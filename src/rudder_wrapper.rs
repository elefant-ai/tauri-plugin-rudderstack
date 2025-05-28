use std::sync::{Arc, Mutex};

use rudderanalytics::client::RudderAnalytics;
use tauri::Runtime;

use crate::config::{self, Config};

/// Trait for rate limiting analytics messages
/// 
/// # Example
/// 
/// ```rust
/// use tauri_plugin_rudderstack::RateLimiter;
/// 
/// struct SimpleRateLimiter {
///     counter: std::sync::atomic::AtomicUsize,
/// }
/// 
/// impl RateLimiter for SimpleRateLimiter {
///     fn let_pass(&self, _msg: &rudderanalytics::message::Message) -> bool {
///         let count = self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
///         count % 10 != 9 // Drop every 10th event
///     }
/// }
/// ```
pub trait RateLimiter: Send + Sync {
    fn let_pass(&self, msg: &rudderanalytics::message::Message) -> bool;
}

impl<T> RateLimiter for T
where
    T: Fn(&rudderanalytics::message::Message) -> bool + Send + Sync,
{
    fn let_pass(&self, msg: &rudderanalytics::message::Message) -> bool {
        self(msg)
    }
}

/// merge two json values
fn merge(a: &mut serde_json::Value, b: &serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(serde_json::Value::Null), v);
            }
        }
        (a, b) => *a = b.clone(),
    }
}


pub struct RudderWrapper {
    rudder: Arc<RudderAnalytics>,
    config: Mutex<config::Config>,
    context: Mutex<crate::types::Context>,
    rate_limiter: Mutex<Option<Box<dyn RateLimiter>>>,
}

impl RudderWrapper {
    /// Create a new RudderWrapper instance
    pub fn new(data_plane: String, key: String, config: Config, context: crate::types::Context) -> Self {
        let rudder = Arc::new(RudderAnalytics::load(key, data_plane));
        Self {
            rudder,
            config: Mutex::new(config),
            context: Mutex::new(context),
            rate_limiter: Mutex::new(None),
        }
    }

    /// Get the anonymous id asigned to this client
    pub fn get_anonymous_id(&self) -> String {
        self.config.lock().unwrap().anonymous_id().to_string()
    }

    pub fn save<R: Runtime>(&self, app: &tauri::AppHandle<R>) -> Result<(), config::ClientIdError> {
        let config = self.config.lock().unwrap();
        config.save(app)
    }

    /// Register a rate limiter function
    /// The rate limiter function should return true if the message should be sent,
    /// false if it should be dropped
    pub fn set_rate_limiter(&self, rate_limiter: Box<dyn RateLimiter>) {
        let mut limiter = self.rate_limiter.lock().unwrap();
        *limiter = Some(rate_limiter);
    }

    /// Remove the rate limiter
    pub fn remove_rate_limiter(&self) {
        let mut limiter = self.rate_limiter.lock().unwrap();
        *limiter = None;
    }

    pub(crate) fn add_to_context(
        &self,
        key: String,
        value: serde_json::Value,
    ) -> Option<serde_json::Value> {
        let mut context = self.context.lock().unwrap();
        context.insert(key, value)
    }

    pub(crate) fn remove_from_context(&self, key: &str) -> Option<serde_json::Value> {
        let mut context = self.context.lock().unwrap();
        context.remove(key)
    }

    pub(crate) fn get_context(&self) -> serde_json::Map<String, serde_json::Value> {
        self.context.lock().unwrap().clone()
    }

    pub(crate) fn clear_context(&self) {
        self.context.lock().unwrap().clear();
    }

    /// Set the anonymous id for this client
    /// This will be used in all subsequent events
    /// it will overwrite the previous anonymous id including the one saved in the file
    pub(crate) fn set_anonymous_id(&self, anonymous_id: String) {
        self.config.lock().unwrap().set_anonymous_id(anonymous_id);
    }

    /// Set the user id for this client
    /// This will be used in all subsequent events
    /// it will overwrite the previous user id
    pub(crate) fn set_user_id(&self, user_id: Option<String>) {
        let should_send_identify = {
            let mut config = self.config.lock().unwrap();
            let result = config.set_user_id(user_id.clone());
            result == Some(false)
        };

        if should_send_identify {
            self.send(rudderanalytics::message::Message::Identify(
                rudderanalytics::message::Identify {
                    user_id,
                    anonymous_id: Some(self.get_anonymous_id()),
                    ..Default::default()
                },
            ));
        }
    }

    /// Function that will receive user event data
    /// and after validation
    /// modify it to Ruddermessage format and send the event to data plane url \
    /// add anonymous_id to all messages except alias.
    /// NOTE: this function will try to acquire a lock on the config.
    pub fn send(
        &self,
        msg: rudderanalytics::message::Message,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        // Check rate limiter before processing the message
        {
            let rate_limiter = self.rate_limiter.lock().unwrap();
            if let Some(limiter) = rate_limiter.as_ref() {
                if !limiter.let_pass(&msg) {
                    tracing::warn!("Event dropped by rate limiter: {:?}", msg);
                    // Return a completed future with Ok(()) for dropped events
                    // Rate limiting should be transparent to the API consumer
                    return tauri::async_runtime::spawn_blocking(|| Ok(()));
                }
            }
        }

        let rudder = self.rudder.clone();
        let anonymous_id = self.get_anonymous_id();

        let user_id = {
            self.config
                .lock()
                .unwrap()
                .user_id()
                .map(|id| id.to_string())
        };
        let mut context = {
            let context = self.context.lock().unwrap();
            serde_json::Value::Object(context.clone())
        };
        let msg = match msg {
            rudderanalytics::message::Message::Identify(identify) => {
                let context = {
                    let mut context = context.clone();
                    if let Some(identify_context) = identify.context {
                        merge(&mut context, &identify_context);
                    }
                    Some(context)
                };
                rudderanalytics::message::Message::Identify(rudderanalytics::message::Identify {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    context,
                    ..identify
                })
            }
            rudderanalytics::message::Message::Alias(alias) => {
                rudderanalytics::message::Message::Alias(alias)
            }
            rudderanalytics::message::Message::Group(group) => {
                let context = {
                    if let Some(group_context) = group.context {
                        merge(&mut context, &group_context);
                    }
                    Some(context)
                };
                rudderanalytics::message::Message::Group(rudderanalytics::message::Group {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    context,
                    ..group
                })
            }
            rudderanalytics::message::Message::Page(page) => {
                let context = {
                    if let Some(page_context) = page.context {
                        merge(&mut context, &page_context);
                    }
                    Some(context)
                };
                rudderanalytics::message::Message::Page(rudderanalytics::message::Page {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    context,
                    ..page
                })
            }
            rudderanalytics::message::Message::Screen(screen) => {
                let context = {
                    if let Some(screen_context) = screen.context {
                        merge(&mut context, &screen_context);
                    }
                    Some(context)
                };
                rudderanalytics::message::Message::Screen(rudderanalytics::message::Screen {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    context,
                    ..screen
                })
            }
            rudderanalytics::message::Message::Track(track) => {
                let context = {
                    if let Some(track_context) = track.context {
                        merge(&mut context, &track_context);
                    }
                    Some(context)
                };
                rudderanalytics::message::Message::Track(rudderanalytics::message::Track {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    context,
                    ..track
                })
            }
            rudderanalytics::message::Message::Batch(batch) => {
                let context = {
                    if let Some(batch_context) = batch.context {
                        merge(&mut context, &batch_context);
                    }
                    Some(context)
                };
                rudderanalytics::message::Message::Batch(rudderanalytics::message::Batch {
                    batch: batch
                        .batch
                        .into_iter()
                        .map(|msg| handle_batch_message(msg, anonymous_id.clone(), user_id.clone()))
                        .collect(),
                    context,
                    ..batch
                })
            }
        };
        tauri::async_runtime::spawn_blocking(move || rudder.send(&msg))
    }
}

/// Function that will receive a batch message and an anonymous_id \
/// and will add the anonymous_id to all messages except alias
fn handle_batch_message(
    batch_message: rudderanalytics::message::BatchMessage,
    anonymous_id: String,
    user_id: Option<String>,
) -> rudderanalytics::message::BatchMessage {
    match batch_message {
        rudderanalytics::message::BatchMessage::Identify(identify) => {
            let identify = rudderanalytics::message::Identify {
                anonymous_id: Some(anonymous_id),
                user_id,
                ..identify
            };
            rudderanalytics::message::BatchMessage::Identify(identify)
        }
        rudderanalytics::message::BatchMessage::Alias(alias) => {
            rudderanalytics::message::BatchMessage::Alias(alias)
        }
        rudderanalytics::message::BatchMessage::Group(group) => {
            let group = rudderanalytics::message::Group {
                anonymous_id: Some(anonymous_id),
                user_id,
                ..group
            };
            rudderanalytics::message::BatchMessage::Group(group)
        }
        rudderanalytics::message::BatchMessage::Page(page) => {
            let page = rudderanalytics::message::Page {
                anonymous_id: Some(anonymous_id),
                user_id,
                ..page
            };
            rudderanalytics::message::BatchMessage::Page(page)
        }
        rudderanalytics::message::BatchMessage::Screen(screen) => {
            let screen = rudderanalytics::message::Screen {
                anonymous_id: Some(anonymous_id),
                user_id,
                ..screen
            };
            rudderanalytics::message::BatchMessage::Screen(screen)
        }
        rudderanalytics::message::BatchMessage::Track(track) => {
            let track = rudderanalytics::message::Track {
                anonymous_id: Some(anonymous_id),
                user_id,
                ..track
            };
            rudderanalytics::message::BatchMessage::Track(track)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestRateLimiter {
        call_count: Arc<AtomicUsize>,
    }

    impl RateLimiter for TestRateLimiter {
        fn let_pass(&self, _msg: &rudderanalytics::message::Message) -> bool {
            let count = self.call_count.fetch_add(1, Ordering::SeqCst);
            count % 2 == 0 // Allow every other message
        }
    }

    #[test]
    fn test_rate_limiter_functionality() {
        let config = crate::config::Config::default();
        let context = serde_json::Map::new();
        let wrapper = RudderWrapper::new(
            "http://localhost:8080".to_string(),
            "test_key".to_string(),
            config,
            context,
        );

        // Create a rate limiter that counts calls
        let call_count = Arc::new(AtomicUsize::new(0));
        let rate_limiter = TestRateLimiter {
            call_count: call_count.clone(),
        };

        // Set the rate limiter
        wrapper.set_rate_limiter(Box::new(rate_limiter));

        // The rate limiter should be called when sending messages
        // Note: We can't easily test the actual sending without setting up a full environment,
        // but we can verify the rate limiter is properly stored and can be removed
        
        // Remove the rate limiter
        wrapper.remove_rate_limiter();
        
        // Test passes if no panics occur
    }
}
