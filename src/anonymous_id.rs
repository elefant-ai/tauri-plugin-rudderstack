use std::path::PathBuf;

use tauri::{AppHandle, Manager, Runtime};

fn get_path<R: Runtime>(handle: &AppHandle<R>) -> Result<PathBuf, ClientIdError> {
    let path = handle.path().app_config_dir()?.join("anonymous-id.txt");
    Ok(path)
}

/// Save the client id to a file.
pub fn save_anonymous_id<R: Runtime>(
    handle: &AppHandle<R>,
    id: String,
) -> Result<(), ClientIdError> {
    tracing::info!("saving anonymous id");
    let path = get_path(handle)?;
    std::fs::write(path, id)?;
    Ok(())
}

/// Get the client id from the file if it exists, otherwise generate a new one.
pub fn get_anonymous_id<R: Runtime>(handle: &AppHandle<R>) -> Result<String, ClientIdError> {
    let path = get_path(handle)?;
    match std::fs::read_to_string(path) {
        Ok(id) => {
            tracing::info!("loaded anonymous id from file");
            Ok(id)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::info!("generating new anonymous id");
            Ok(uuid::Uuid::new_v4().to_string())
        }
        Err(e) => Err(e.into()),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientIdError {
    #[error("failed to save client id {0}")]
    Save(#[from] std::io::Error),
    #[error("failed to get app config dir")]
    AppConfigDir(#[from] tauri::Error),
}
