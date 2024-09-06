use tauri::{AppHandle, Runtime};
use tracing::error;

use crate::{
    types::{Alias, Group, Identify, Page, Screen, Track},
    AnalyticsExt as _,
};

macro_rules! handle_error {
    ($result:expr) => {
        match $result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                error!("Failed to send analytics event: {:?}", e);
            }
            Err(e) => {
                error!("Failed to send analytics event: {:?}", e);
            }
        }
    };
}

#[tauri::command]
#[specta::specta]
/// Send an analytics event to the RudderStack data plane.
pub async fn send_analytics_alias<R: Runtime>(app: AppHandle<R>, event: Alias) {
    handle_error!(app.send_analytic_alias(event).await);
}

#[tauri::command]
#[specta::specta]
/// Send an analytics event to the RudderStack data plane.
pub async fn send_analytics_group<R: Runtime>(app: AppHandle<R>, event: Group) {
    handle_error!(app.send_analytic_group(event).await);
}

#[tauri::command]
#[specta::specta]
/// Send an [Identify] event to the RudderStack data plane.
pub async fn send_analytics_identify<R: Runtime>(app: AppHandle<R>, event: Identify) {
    handle_error!(app.send_analytic_identify(event).await);
}

#[tauri::command]
#[specta::specta]
/// Send a [Page] event to the RudderStack data plane.
pub async fn send_analytics_page<R: Runtime>(app: AppHandle<R>, event: Page) {
    handle_error!(app.send_analytic_page(event).await);
}

#[tauri::command]
#[specta::specta]
/// Send a [Screen] event to the RudderStack data plane.
pub async fn send_analytics_screen<R: Runtime>(app: AppHandle<R>, event: Screen) {
    handle_error!(app.send_analytic_screen(event).await);
}

#[tauri::command]
#[specta::specta]
/// Send a [Track] event to the RudderStack data plane.
pub async fn send_analytics_track<R: Runtime>(app: AppHandle<R>, event: Track) {
    handle_error!(app.send_analytic_track(event).await);
}
