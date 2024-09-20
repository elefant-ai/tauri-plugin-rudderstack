use std::sync::{Arc, Mutex};

use rudderanalytics::client::RudderAnalytics;
use tauri::Runtime;

use crate::config::{self, Config};

pub struct RudderWrapper {
    rudder: Arc<RudderAnalytics>,
    config: Mutex<config::Config>,
}

impl RudderWrapper {
    /// Create a new RudderWrapper instance
    pub fn new(data_plane: String, key: String, config: Config) -> Self {
        let rudder = Arc::new(RudderAnalytics::load(key, data_plane));
        Self {
            rudder,
            config: Mutex::new(config),
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
            tracing::debug!("set_user_id: sending identify event");
        } else {
            tracing::debug!("set_user_id: not sending identify event");
        }

        if should_send_identify {
            self.send(rudderanalytics::message::Message::Identify(rudderanalytics::message::Identify {
                user_id,
                anonymous_id: Some(self.get_anonymous_id()),
                ..Default::default()
            }));
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
        let rudder = self.rudder.clone();
        let anonymous_id = self.get_anonymous_id();
        
        let user_id = {self.config.lock().unwrap().user_id().map(|id| id.to_string())};
        let msg = match msg {
            rudderanalytics::message::Message::Identify(identify) => {
                rudderanalytics::message::Message::Identify(rudderanalytics::message::Identify {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    ..identify
                })
            }
            rudderanalytics::message::Message::Alias(alias) => {
                rudderanalytics::message::Message::Alias(alias)
            }
            rudderanalytics::message::Message::Group(group) => {
                rudderanalytics::message::Message::Group(rudderanalytics::message::Group {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    ..group
                })
            }
            rudderanalytics::message::Message::Page(page) => {
                rudderanalytics::message::Message::Page(rudderanalytics::message::Page {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    ..page
                })
            }
            rudderanalytics::message::Message::Screen(screen) => {
                rudderanalytics::message::Message::Screen(rudderanalytics::message::Screen {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    ..screen
                })
            }
            rudderanalytics::message::Message::Track(track) => {
                rudderanalytics::message::Message::Track(rudderanalytics::message::Track {
                    anonymous_id: Some(anonymous_id),
                    user_id,
                    ..track
                })
            }
            rudderanalytics::message::Message::Batch(batch) => {
                rudderanalytics::message::Message::Batch(rudderanalytics::message::Batch {
                    batch: batch
                        .batch
                        .into_iter()
                        .map(|msg| handle_batch_message(msg, anonymous_id.clone(),user_id.clone()))
                        .collect(),
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
