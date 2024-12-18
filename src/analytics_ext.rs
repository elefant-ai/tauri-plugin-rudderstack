use tauri::{Manager as _, Runtime};

use crate::{
    config,
    rudder_wrapper::RudderWrapper,
    types::{self, Alias, Group, Identify, Page, Screen, Track},
};

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the analytics APIs.
pub trait AnalyticsExt<R: Runtime> {
    /// Send an analytics event to the RudderStack data plane.
    fn send_analytic(
        &self,
        event: types::Message,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>>;

    /// Send an [Identify] event to the RudderStack data plane.
    fn send_analytic_identify(
        &self,
        event: Identify,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        let event = types::Message::Identify(event);
        self.send_analytic(event)
    }

    /// Send a [Track] event to the RudderStack data plane.
    fn send_analytic_track(
        &self,
        event: Track,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        let event = types::Message::Track(event);
        self.send_analytic(event)
    }

    /// Send a [Page] event to the RudderStack data plane.
    fn send_analytic_page(
        &self,
        event: Page,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        let event = types::Message::Page(event);
        self.send_analytic(event)
    }

    /// Send a [Screen] event to the RudderStack data plane.
    fn send_analytic_screen(
        &self,
        event: Screen,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        let event = types::Message::Screen(event);
        self.send_analytic(event)
    }

    /// Send a [Group] event to the RudderStack data plane.
    fn send_analytic_group(
        &self,
        event: Group,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        let event = types::Message::Group(event);
        self.send_analytic(event)
    }

    /// Send an [Alias] event to the RudderStack data plane.
    fn send_analytic_alias(
        &self,
        event: Alias,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        let event = types::Message::Alias(event);
        self.send_analytic(event)
    }

    /// Set the anonymous ID of the user. This will be used in all subsequent events.
    /// It will overwrite the previous anonymous ID including the one saved in the file.
    fn set_anonymous_id(&self, id: String) -> Result<(), config::ClientIdError>;

    /// Set the user ID of the user. This will be used in all subsequent events.
    /// It will overwrite the previous user ID.
    fn set_user_id(&self, id: Option<String>);

    /// Set the OS of the user. This will be used in all subsequent events.
    fn set_os(&self, os: Option<String>);

    /// Set the app version of the user. This will be used in all subsequent events.
    fn set_app_version(&self, app_version: Option<String>);
}

impl<R: Runtime> AnalyticsExt<R> for tauri::AppHandle<R> {
    fn send_analytic(
        &self,
        event: types::Message,
    ) -> tauri::async_runtime::JoinHandle<Result<(), rudderanalytics::errors::Error>> {
        tracing::trace!("sending analytics event: {:?}", event);
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

    fn set_os(&self, os: Option<String>) {
        tracing::debug!("setting os: {:?}", os);
        let rudder = self.state::<RudderWrapper>();
        rudder.set_os(os);
    }

    fn set_app_version(&self, app_version: Option<String>) {
        tracing::debug!("setting app version: {:?}", app_version);
        let rudder = self.state::<RudderWrapper>();
        rudder.set_app_version(app_version);
    }
}
