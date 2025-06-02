use tauri::{Manager as _, Runtime};

use crate::{
    config,
    rudder_wrapper::{RateLimiter, RudderWrapper},
    types::{self, Alias, Group, Identify, Page, Screen, Track},
};

/// The result of sending an analytics event.
pub enum SendResult {
    /// the event was dropped by the rate limiter
    EventDropped,
    /// Thread handle
    ThreadHandle(tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>>),
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the analytics APIs.
pub trait AnalyticsExt<R: Runtime> {
    /// Send an analytics event to the RudderStack data plane.
    fn send_analytic(&self, event: types::Message) -> SendResult;

    /// Send an [Identify] event to the RudderStack data plane.
    fn send_analytic_identify(&self, event: Identify) -> SendResult {
        let event = types::Message::Identify(event);
        self.send_analytic(event)
    }

    /// Send a [Track] event to the RudderStack data plane.
    fn send_analytic_track(&self, event: Track) -> SendResult {
        let event = types::Message::Track(event);
        self.send_analytic(event)
    }

    /// Send a [Page] event to the RudderStack data plane.
    fn send_analytic_page(&self, event: Page) -> SendResult {
        let event = types::Message::Page(event);
        self.send_analytic(event)
    }

    /// Send a [Screen] event to the RudderStack data plane.
    fn send_analytic_screen(&self, event: Screen) -> SendResult {
        let event = types::Message::Screen(event);
        self.send_analytic(event)
    }

    /// Send a [Group] event to the RudderStack data plane.
    fn send_analytic_group(&self, event: Group) -> SendResult {
        let event = types::Message::Group(event);
        self.send_analytic(event)
    }

    /// Send an [Alias] event to the RudderStack data plane.
    fn send_analytic_alias(&self, event: Alias) -> SendResult {
        let event = types::Message::Alias(event);
        self.send_analytic(event)
    }

    /// Set the anonymous ID of the user. This will be used in all subsequent events.
    /// It will overwrite the previous anonymous ID including the one saved in the file.
    fn set_anonymous_id(&self, id: String) -> Result<(), config::ClientIdError>;

    /// Set the user ID of the user. This will be used in all subsequent events.
    /// It will overwrite the previous user ID.
    fn set_user_id(&self, id: Option<String>);

    /// Add to context hash map
    fn add_to_context(&self, key: String, value: serde_json::Value) -> Option<serde_json::Value>;

    /// Remove from context hash map
    fn remove_from_context(&self, key: &str) -> Option<serde_json::Value>;

    /// Clear the context hash map
    fn clear_context(&self);

    /// Get the context hash map
    fn get_context(&self) -> crate::types::Context;

    /// Register a rate limiter function
    /// The rate limiter function should return true if the message should be sent,
    /// false if it should be dropped
    fn set_rate_limiter(&self, rate_limiter: impl RateLimiter + 'static);

    /// Remove the rate limiter
    fn remove_rate_limiter(&self);
}

impl<R: Runtime> AnalyticsExt<R> for tauri::AppHandle<R> {
    fn send_analytic(&self, event: types::Message) -> SendResult {
        tracing::trace!(event = ?event, "sending analytics event");
        tracing::debug!("sending analytics event");
        let message = types::convert_message(event);
        let rudder = self.state::<RudderWrapper>();
        rudder.send(message)
    }

    fn set_anonymous_id(&self, id: String) -> Result<(), config::ClientIdError> {
        tracing::debug!("setting anonymous id: {:?}", id);
        let rudder = self.state::<RudderWrapper>();
        rudder.set_anonymous_id(id.clone());
        rudder.save(self)
    }

    fn set_user_id(&self, id: Option<String>) {
        tracing::debug!("setting user id: {:?}", id);
        let rudder = self.state::<RudderWrapper>();
        rudder.set_user_id(id.clone());
    }

    fn add_to_context(&self, key: String, value: serde_json::Value) -> Option<serde_json::Value> {
        tracing::debug!("adding to context: {:?} -> {:?}", key, value);
        let rudder = self.state::<RudderWrapper>();
        rudder.add_to_context(key, value)
    }

    fn remove_from_context(&self, key: &str) -> Option<serde_json::Value> {
        tracing::debug!("removing from context: {:?}", key);
        let rudder = self.state::<RudderWrapper>();
        rudder.remove_from_context(key)
    }

    fn clear_context(&self) {
        tracing::debug!("clearing context");
        let rudder = self.state::<RudderWrapper>();
        rudder.clear_context();
    }

    fn get_context(&self) -> crate::types::Context {
        tracing::debug!("getting context");
        let rudder = self.state::<RudderWrapper>();
        rudder.get_context()
    }

    fn set_rate_limiter(&self, rate_limiter: impl RateLimiter + 'static) {
        tracing::debug!("setting rate limiter");
        let rudder = self.state::<RudderWrapper>();
        rudder.set_rate_limiter(Box::new(rate_limiter));
    }

    fn remove_rate_limiter(&self) {
        tracing::debug!("removing rate limiter");
        let rudder = self.state::<RudderWrapper>();
        rudder.remove_rate_limiter();
    }
}

impl<R: Runtime> AnalyticsExt<R> for tauri::App<R> {
    fn send_analytic(&self, event: types::Message) -> SendResult {
        self.handle().send_analytic(event)
    }

    fn set_anonymous_id(&self, id: String) -> Result<(), config::ClientIdError> {
        self.handle().set_anonymous_id(id)
    }

    fn set_user_id(&self, id: Option<String>) {
        self.handle().set_user_id(id)
    }

    fn add_to_context(&self, key: String, value: serde_json::Value) -> Option<serde_json::Value> {
        self.handle().add_to_context(key, value)
    }

    fn remove_from_context(&self, key: &str) -> Option<serde_json::Value> {
        self.handle().remove_from_context(key)
    }

    fn clear_context(&self) {
        self.handle().clear_context()
    }

    fn get_context(&self) -> crate::types::Context {
        self.handle().get_context()
    }

    fn set_rate_limiter(&self, rate_limiter: impl RateLimiter + 'static) {
        self.handle().set_rate_limiter(rate_limiter)
    }

    fn remove_rate_limiter(&self) {
        self.handle().remove_rate_limiter()
    }
}

impl<R: Runtime> AnalyticsExt<R> for tauri::Window<R> {
    fn send_analytic(&self, event: types::Message) -> SendResult {
        self.app_handle().send_analytic(event)
    }

    fn set_anonymous_id(&self, id: String) -> Result<(), config::ClientIdError> {
        self.app_handle().set_anonymous_id(id)
    }

    fn set_user_id(&self, id: Option<String>) {
        self.app_handle().set_user_id(id)
    }

    fn add_to_context(&self, key: String, value: serde_json::Value) -> Option<serde_json::Value> {
        self.app_handle().add_to_context(key, value)
    }

    fn remove_from_context(&self, key: &str) -> Option<serde_json::Value> {
        self.app_handle().remove_from_context(key)
    }

    fn clear_context(&self) {
        self.app_handle().clear_context()
    }

    fn get_context(&self) -> crate::types::Context {
        self.app_handle().get_context()
    }

    fn set_rate_limiter(&self, rate_limiter: impl RateLimiter + 'static) {
        self.app_handle().set_rate_limiter(rate_limiter)
    }

    fn remove_rate_limiter(&self) {
        self.app_handle().remove_rate_limiter()
    }
}
