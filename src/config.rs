use std::{collections::HashMap, path::PathBuf};

use tauri::{AppHandle, Manager, Runtime};
use tracing::debug;


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// The anonymous ID of the user. this is normally generated and saved in the file.
    anonymous_id: String,
    /// A map of user IDs to anonymous IDs. this is used to connect the user ID to the anonymous ID.
    connected_ids: HashMap<String, String>,
    /// The user ID of the user. this is used to identify the user.
    user_id: Option<String>,
    /// The OS used.
    os: Option<String>,
    /// The app version.
    app_version: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            anonymous_id: uuid::Uuid::new_v4().to_string(),
            connected_ids: HashMap::new(),
            user_id: None,
            os: None,
            app_version: None,
        }
    }
}

impl Config {
    pub fn new(anonymous_id: String) -> Self {
        Self {
            anonymous_id,
            connected_ids: HashMap::new(),
            user_id: None,
            os: None,
            app_version: None,
        }
    }

    /// Get the anonymous ID of the user.
    pub fn anonymous_id(&self) -> &str {
        &self.anonymous_id
    }

    /// Set the anonymous ID of the user. 
    pub fn set_anonymous_id(&mut self, anonymous_id: String) {
        self.anonymous_id = anonymous_id;
    }

    /// Get the user ID of the user.
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    /// Set the user ID of the user. \
    /// if the user ID is passed in is None, it will return None. \
    /// if the user ID is passed in is Some, it will return Some(true) if the user ID is already connected to the anonymous ID. \
    /// if the user ID is passed in is Some, it will return Some(false) if the user ID is not connected to the anonymous ID.
    pub fn set_user_id(&mut self, user_id: Option<String>) -> Option<bool>  {
        self.user_id = user_id.clone();
        if let Some(id) = user_id {
            if self.connected_ids.contains_key(&id) {
                Some(true)
            } else {
                self.connected_ids.insert(id, self.anonymous_id.clone());
                Some(false)
            }
            
        } else {
            None
        }
    }

    pub fn set_os(&mut self, os: Option<String>) {
        self.os = os;
    }

    pub fn set_app_version(&mut self, app_version: Option<String>) {
        self.app_version = app_version;
    }

    /// Save the config to a file.
    pub fn save<R: Runtime>(&self, handle: &AppHandle<R>) -> Result<(), ClientIdError> {
        debug!("saving config");
        let path = Self::get_path(handle)?;
        let config = serde_json::to_vec(&self)?;
        Ok(std::fs::write(&path, config)?)
    }

    /// Load the config from a file.
    pub fn load<R: Runtime>(handle: &AppHandle<R>) -> Self {
        debug!("loading config");

        Self::try_load(handle).unwrap_or_default()
    }

    fn try_load<R: Runtime>(handle: &AppHandle<R>) -> Result<Self, ClientIdError> {
        let path = Self::get_path(handle)?;
        let config = std::fs::read(&path)?;
        Ok(serde_json::from_slice(&config)?)
    }

    fn get_path<R: Runtime>(handle: &AppHandle<R>) -> Result<PathBuf, ClientIdError> {
        let path = handle
            .path()
            .app_config_dir()?
            .join("tauri-rudderstack.json");
        Ok(path)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientIdError {
    #[error("failed to save client id {0}")]
    Save(#[from] std::io::Error),
    #[error("failed to get app config dir")]
    AppConfigDir(#[from] tauri::Error),
    #[error("failed to serialize config")]
    Serialize(#[from] serde_json::Error),
}
