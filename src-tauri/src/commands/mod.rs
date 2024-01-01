use std::{ collections::HashMap, fmt::Display };

use crate::installer::{ studio, download_from_repo, player };

type Result<T> = std::result::Result<T, String>;

/* Make it so javascript can read our errors */
fn convert_err<T, V: Display>(check: std::result::Result<T, V>) -> Result<T> {
    match check {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
pub async fn get_available_studio() -> Result<HashMap<String, String>> {
    convert_err(studio::get_available().await)
}

#[tauri::command]
pub fn studio_installed(year: &str) -> bool {
    return studio::is_installed(year);
}

#[tauri::command]
pub async fn install_studio(year: &str, url: &str) -> Result<()> {
    convert_err(studio::download_studio(year, url).await)
}

#[tauri::command]
pub async fn get_valid_clients() -> Result<Vec<String>> {
    convert_err(player::get_valid_clients().await)
}

#[tauri::command]
pub async fn get_client_manifest(year: &str) -> Result<HashMap<String, String>> {
    convert_err(player::get_client_manifest(year).await)
}
