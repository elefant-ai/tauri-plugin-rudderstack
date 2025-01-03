#![doc = include_str!("../README.md")]

pub use analytics_ext::AnalyticsExt;
use rudder_wrapper::RudderWrapper;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, RunEvent, Runtime,
};
use tracing::{error, info};
use types::Track;

mod analytics_ext;
mod commands;
mod config;
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

pub struct RudderStackBuilder {
    data_plane: String,
    key: String,
    anonymous_id: Option<String>,
    first_run: bool,
    context: types::Context,
}

impl RudderStackBuilder {
    /// Initializes the plugin.
    ///
    /// # Parameters
    /// - `data_plane`: The URL of the RudderStack data plane.
    /// - `key`: The write key of the RudderStack project.
    pub fn new(data_plane: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            data_plane: data_plane.into(),
            key: key.into(),
            anonymous_id: None,
            first_run: false,
            context: serde_json::Map::new(),
        }
    }

    /// WARNING: This will stop the internal anonymous ID from being generated.
    ///
    /// The anonymous ID of the user. this is optional and will be generated if not provided. if provided it will need to be provided on subsequent runs to maintain the same user.
    pub fn anonymous_id(mut self, id: impl Into<String>) -> Self {
        self.anonymous_id = Some(id.into());
        self
    }

    /// If set to true, the plugin will send a first run event on the first run.
    pub fn first_run(mut self, first_run: bool) -> Self {
        self.first_run = first_run;
        self
    }

    /// Allows you to set the context that will be sent with every event.
    pub fn with_context<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut serde_json::Map<String, serde_json::Value>),
    {
        f(&mut self.context);
        self
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        info!("Initializing RudderStack plugin");
        let specta = init_commands();
        Builder::new(PLUGIN_NAME)
            .invoke_handler(specta.invoke_handler())
            .setup(move |app, _| {
                // load the config from the file or create a new one
                let config = config::Config::try_load(app);

                // if first run is set, and loading the config failed, set the first run flag since a new uuid will be generated
                let first_run = self.first_run && config.is_err();

                let mut config = config.unwrap_or_default();

                // set the anonymous id if provided
                if let Some(id) = self.anonymous_id {
                    config.set_anonymous_id(id);
                };
                // save the config
                if let Err(err) = config.save(app) {
                    error!("Failed to save config: {:?}", err);
                }
                let rudder_analytics = RudderWrapper::new(self.data_plane, self.key, config, self.context);

                app.manage(rudder_analytics);

                if first_run {
                    app.send_analytic_track(types::Track {
                        event: "First Run".to_string(),
                        ..Track::default()
                    });
                }

                Ok(())
            })
            .on_event(|app, event| {
                if let RunEvent::Exit = event {
                    let host = app.state::<RudderWrapper>();
                    if let Err(err) = host.save(app) {
                        error!("Failed to save config: {:?}", err);
                    }
                }
            })
            .build()
    }
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
