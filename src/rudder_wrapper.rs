use std::sync::{Arc, Mutex};

use rudderanalytics::client::RudderAnalytics;

pub struct RudderWrapper {
    rudder: Arc<RudderAnalytics>,
    anonymous_id: Mutex<String>,
}

impl RudderWrapper {
    /// Create a new RudderWrapper instance
    pub fn new(data_plane: String, key: String, anonymous_id: String) -> Self {
        let rudder = Arc::new(RudderAnalytics::load(key, data_plane));
        Self {
            rudder,
            anonymous_id: Mutex::new(anonymous_id),
        }
    }

    /// Get the anonymous id asigned to this client
    pub fn get_anonymous_id(&self) -> String {
        self.anonymous_id.lock().unwrap().clone()
    }

    /// Set the anonymous id for this client
    /// This will be used in all subsequent events
    /// it will overwrite the previous anonymous id including the one saved in the file
    pub(crate) fn set_anonymous_id(&self, anonymous_id: String) {
        *self.anonymous_id.lock().unwrap() = anonymous_id;
    }

    /// Function that will receive user event data
    /// and after validation
    /// modify it to Ruddermessage format and send the event to data plane url \
    /// add anonymous_id to all messages except alias
    pub fn send(
        &self,
        msg: rudderanalytics::message::Message,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        let rudder = self.rudder.clone();
        let anonymous_id = {
            let id = self.anonymous_id.lock().unwrap();
            id.clone()
        };
        let msg = match msg {
            rudderanalytics::message::Message::Identify(identify) => {
                rudderanalytics::message::Message::Identify(rudderanalytics::message::Identify {
                    anonymous_id: Some(anonymous_id),
                    ..identify
                })
            }
            rudderanalytics::message::Message::Alias(alias) => {
                rudderanalytics::message::Message::Alias(alias)
            }
            rudderanalytics::message::Message::Group(group) => {
                rudderanalytics::message::Message::Group(rudderanalytics::message::Group {
                    anonymous_id: Some(anonymous_id),
                    ..group
                })
            }
            rudderanalytics::message::Message::Page(page) => {
                rudderanalytics::message::Message::Page(rudderanalytics::message::Page {
                    anonymous_id: Some(anonymous_id),
                    ..page
                })
            }
            rudderanalytics::message::Message::Screen(screen) => {
                rudderanalytics::message::Message::Screen(rudderanalytics::message::Screen {
                    anonymous_id: Some(anonymous_id),
                    ..screen
                })
            }
            rudderanalytics::message::Message::Track(track) => {
                rudderanalytics::message::Message::Track(rudderanalytics::message::Track {
                    anonymous_id: Some(anonymous_id),
                    ..track
                })
            }
            rudderanalytics::message::Message::Batch(batch) => {
                rudderanalytics::message::Message::Batch(rudderanalytics::message::Batch {
                    batch: batch
                        .batch
                        .into_iter()
                        .map(|msg| handle_batch_message(msg, anonymous_id.clone()))
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
) -> rudderanalytics::message::BatchMessage {
    match batch_message {
        rudderanalytics::message::BatchMessage::Identify(identify) => {
            let identify = rudderanalytics::message::Identify {
                anonymous_id: Some(anonymous_id),
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
                ..group
            };
            rudderanalytics::message::BatchMessage::Group(group)
        }
        rudderanalytics::message::BatchMessage::Page(page) => {
            let page = rudderanalytics::message::Page {
                anonymous_id: Some(anonymous_id),
                ..page
            };
            rudderanalytics::message::BatchMessage::Page(page)
        }
        rudderanalytics::message::BatchMessage::Screen(screen) => {
            let screen = rudderanalytics::message::Screen {
                anonymous_id: Some(anonymous_id),
                ..screen
            };
            rudderanalytics::message::BatchMessage::Screen(screen)
        }
        rudderanalytics::message::BatchMessage::Track(track) => {
            let track = rudderanalytics::message::Track {
                anonymous_id: Some(anonymous_id),
                ..track
            };
            rudderanalytics::message::BatchMessage::Track(track)
        }
    }
}