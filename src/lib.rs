pub use analytics_ext::AnalyticsExt;
use rudder_wrapper::RudderWrapper;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, RunEvent, Runtime,
};
use tracing::info;

mod analytics_ext;
mod anonymous_id;
mod commands;
mod rudder_wrapper;
pub mod types;

const PLUGIN_NAME: &str = "rudderstack";

fn init_commands<R: Runtime>() -> tauri_specta::Builder<R> {
    tauri_specta::Builder::new()
        .plugin_name(PLUGIN_NAME)
        .commands(tauri_specta::collect_commands![
            commands::send_analytics_alias<tauri::Wry>,
            commands::send_analytics_group<tauri::Wry>,
            commands::send_analytics_identify<tauri::Wry>,
            commands::send_analytics_page<tauri::Wry>,
            commands::send_analytics_screen<tauri::Wry>,
            commands::send_analytics_track<tauri::Wry>
        ])
}

/// Initializes the plugin.
/// This function should be called in the [`tauri::Builder`] chain.
/// # Parameters
/// - `data_plane`: The URL of the RudderStack data plane.
/// - `key`: The write key of the RudderStack project.
/// - `anonymous_id`: The anonymous ID of the user. this is optional and will be generated if not provided. if provided it will need to be provided on subsequent runs to maintain the same user.
pub fn init<R: Runtime>(
    data_plane: impl Into<String>,
    key: impl Into<String>,
    anonymous_id: Option<String>,
) -> TauriPlugin<R> {
    info!("Initializing RudderStack plugin");
    let data_plane: String = data_plane.into();
    let key: String = key.into();
    let specta = init_commands();
    Builder::new(PLUGIN_NAME)
        .invoke_handler(specta.invoke_handler())
        .setup(|app, _| {
            let anonymous_id = if let Some(id) = anonymous_id {
                id
            } else {
                anonymous_id::get_anonymous_id(app)?
            };
            let rudder_analytics = RudderWrapper::new(data_plane, key, anonymous_id);

            if let Err(e) = anonymous_id::save_anonymous_id(
                app,
                rudder_analytics.get_anonymous_id().to_string(),
            ) {
                tracing::error!("Failed to save anonymous id: {:?}", e);
            }
            app.manage(rudder_analytics);

            Ok(())
        })
        .on_event(|app, event| {
            if let RunEvent::Exit = event {
                let host = app.state::<RudderWrapper>();
                let anonymous_id = host.get_anonymous_id();
                anonymous_id::save_anonymous_id(app, anonymous_id.to_string()).unwrap();
            }
        })
        .build()
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    /// build the commands and export the types
    /// this is used to generate the typescript bindings
    /// this is run with
    ///
    /// `cargo test export_types --features build-types`
    #[test]
    #[cfg(feature = "build-types")]
    fn export_types() {
        use super::*;

        let builder = init_commands::<tauri::Wry>();
        builder
            .export(
                specta_typescript::Typescript::default()
                    .formatter(specta_typescript::formatter::eslint)
                    .bigint(specta_typescript::BigIntExportBehavior::BigInt)
                    .header("/* eslint-disable */ \n // @ts-nocheck"),
                "./guest-js/bindings.ts",
            )
            .expect("failed to export specta types");
    }
}
