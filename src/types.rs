//! This module contains the types used by the RudderStack plugin.
//! These types are mapped to the types used by the RudderStack API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An enum containing all values which may be sent to RudderStack's API.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, specta::Type)]
pub enum Message {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
    Batch(Batch),
}

/// An identify event.
/// The identify call lets you identify a visiting user and associate them to their actions. It also lets you record the traits about them like their name, email address, etc.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct Identify {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The traits to assign to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
   #[serde(rename="originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A track event.
/// The track call lets you record the user actions along with their associated properties. Each user action is called an event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct Track {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The name of the event being tracked.
    pub event: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
   #[serde(rename="originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A page event.
///
/// The page call allows you to record the page views on your website along with the other relevant information about the viewed page.
/// RudderStack recommends calling page at least once every page load.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct Page {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The name of the page being tracked.
    pub name: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
   #[serde(rename="originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A screen event.
///
/// The screen call is the mobile equivalent of the page call.
///
/// The screen method lets you record whenever the user views their mobile screen, along with any additional relevant information about the screen.
/// The screen call is the mobile equivalent of the page call.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct Screen {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The name of the screen being tracked.
    pub name: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
   #[serde(rename="originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A group event.
/// The `group` call lets you associate an identified user to a group - either a company, project or a team and record any custom traits or properties associated with that group. \
/// An identified user can be in more than one group.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct Group {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The group the user is being associated with.
    #[serde(rename = "groupId")]
    pub group_id: String,

    /// The traits to assign to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
   #[serde(rename="originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// An alias event.
///
/// The `alias` call lets you merge different identities of a known user. \
///
/// Alis is an advanced method that lets you change the tracked user's ID explicitly. This method is useful when managing identities for some of the downstream destinations.
///
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct Alias {
    /// The user id associated with this message.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// The user's previous ID.
    #[serde(rename = "previousId")]
    pub previous_id: String,

    /// The traits to assign to the alias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
   #[serde(rename="originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A batch of events.
/// The [Batch] call lets you send multiple user events(of type [Identify], [Track], [Page], [Screen], [Group], and [Alias]) in one call.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default, specta::Type)]
pub struct Batch {
    /// The batch of messages to send.
    pub batch: Vec<BatchMessage>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// The timestamp associated with this message.
   #[serde(rename="originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,
}

/// An enum containing all messages which may be placed inside a batch.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum BatchMessage {
    #[serde(rename = "identify")]
    Identify(Identify),
    #[serde(rename = "track")]
    Track(Track),
    #[serde(rename = "page")]
    Page(Page),
    #[serde(rename = "screen")]
    Screen(Screen),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "alias")]
    Alias(Alias),
}

/// Converts a [Message] to a [rudderanalytics::message::Message].
pub(crate) fn convert_message(message: Message) -> rudderanalytics::message::Message {
    match message {
        Message::Alias(alias) => {
            rudderanalytics::message::Message::Alias(rudderanalytics::message::Alias {
                user_id: alias.user_id,
                previous_id: alias.previous_id,
                traits: alias.traits,
                original_timestamp: alias.original_timestamp,
                context: alias.context,
                integrations: alias.integrations,
            })
        }
        Message::Batch(batch) => {
            rudderanalytics::message::Message::Batch(rudderanalytics::message::Batch {
                batch: batch.batch.into_iter().map(convert_batch_message).collect(),
                context: batch.context,
                integrations: batch.integrations,
                original_timestamp: batch.original_timestamp,
            })
        }
        Message::Group(group) => {
            rudderanalytics::message::Message::Group(rudderanalytics::message::Group {
                user_id: group.user_id,
                anonymous_id: None,
                group_id: group.group_id,
                traits: group.traits,
                original_timestamp: group.original_timestamp,
                context: group.context,
                integrations: group.integrations,
            })
        }
        Message::Identify(identify) => {
            rudderanalytics::message::Message::Identify(rudderanalytics::message::Identify {
                user_id: identify.user_id,
                anonymous_id: None,
                traits: identify.traits,
                original_timestamp: identify.original_timestamp,
                context: identify.context,
                integrations: identify.integrations,
            })
        }
        Message::Page(page) => {
            rudderanalytics::message::Message::Page(rudderanalytics::message::Page {
                user_id: page.user_id,
                anonymous_id: None,
                name: page.name,
                properties: page.properties,
                original_timestamp: page.original_timestamp,
                context: page.context,
                integrations: page.integrations,
            })
        }
        Message::Screen(screen) => {
            rudderanalytics::message::Message::Screen(rudderanalytics::message::Screen {
                user_id: screen.user_id,
                anonymous_id: None,
                name: screen.name,
                properties: screen.properties,
                original_timestamp: screen.original_timestamp,
                context: screen.context,
                integrations: screen.integrations,
            })
        }
        Message::Track(track) => {
            rudderanalytics::message::Message::Track(rudderanalytics::message::Track {
                user_id: track.user_id,
                anonymous_id: None,
                event: track.event,
                properties: track.properties,
                original_timestamp: track.original_timestamp,
                context: track.context,
                integrations: track.integrations,
            })
        }
    }
}

/// Converts a [BatchMessage] to a [rudderanalytics::message::BatchMessage].
fn convert_batch_message(batch_message: BatchMessage) -> rudderanalytics::message::BatchMessage {
    match batch_message {
        BatchMessage::Alias(alias) => {
            rudderanalytics::message::BatchMessage::Alias(rudderanalytics::message::Alias {
                user_id: alias.user_id,
                previous_id: alias.previous_id,
                traits: alias.traits,
                original_timestamp: alias.original_timestamp,
                context: alias.context,
                integrations: alias.integrations,
            })
        }
        BatchMessage::Group(group) => {
            rudderanalytics::message::BatchMessage::Group(rudderanalytics::message::Group {
                user_id: group.user_id,
                anonymous_id: None,
                group_id: group.group_id,
                traits: group.traits,
                original_timestamp: group.original_timestamp,
                context: group.context,
                integrations: group.integrations,
            })
        }
        BatchMessage::Identify(identify) => {
            rudderanalytics::message::BatchMessage::Identify(rudderanalytics::message::Identify {
                user_id: identify.user_id,
                anonymous_id: None,
                traits: identify.traits,
                original_timestamp: identify.original_timestamp,
                context: identify.context,
                integrations: identify.integrations,
            })
        }
        BatchMessage::Page(page) => {
            rudderanalytics::message::BatchMessage::Page(rudderanalytics::message::Page {
                user_id: page.user_id,
                anonymous_id: None,
                name: page.name,
                properties: page.properties,
                original_timestamp: page.original_timestamp,
                context: page.context,
                integrations: page.integrations,
            })
        }
        BatchMessage::Screen(screen) => {
            rudderanalytics::message::BatchMessage::Screen(rudderanalytics::message::Screen {
                user_id: screen.user_id,
                anonymous_id: None,
                name: screen.name,
                properties: screen.properties,
                original_timestamp: screen.original_timestamp,
                context: screen.context,
                integrations: screen.integrations,
            })
        }
        BatchMessage::Track(track) => {
            rudderanalytics::message::BatchMessage::Track(rudderanalytics::message::Track {
                user_id: track.user_id,
                anonymous_id: None,
                event: track.event,
                properties: track.properties,
                original_timestamp: track.original_timestamp,
                context: track.context,
                integrations: track.integrations,
            })
        }
    }
}
