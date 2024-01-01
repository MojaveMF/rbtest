use std::{ collections::HashMap, fmt::Display };

use tauri::api::version;

use crate::installer::{
    studio,
    download_from_repo,
    player,
    self,
    paths::get_downloads_folder,
    SETUP_URL,
    download_file,
};

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

#[tauri::command]
pub async fn download_zip(file_name: &str) -> Result<()> {
    let download_url = format!("https://{}/{}", SETUP_URL, file_name);
    let download_folder = convert_err(get_downloads_folder())?;
    let download_path = download_folder.join(file_name);

    convert_err(download_file(download_url, download_path).await)
}

#[tauri::command]
pub async fn extract_zip(file_name: &str, location: &str) -> Result<()> {
    let download_folder = convert_err(get_downloads_folder())?;
    let zip_path = download_folder.join(file_name);

    convert_err(installer::extract_zip(zip_path, location).await)
}

#[tauri::command]
pub async fn get_latest_version() -> Result<String> {
    convert_err(installer::latest_version().await)
}

#[tauri::command]
pub async fn prepare_client(
    year: &str,
    version: &str,
    manifest: HashMap<String, String>
) -> Result<()> {
    println!("{:?}", manifest);
    convert_err(player::prepare_client(year, version, manifest).await)
}

#[tauri::command]
pub fn client_installed(year: &str, version: &str) -> bool {
    player::installed(year, version)
}

#[tauri::command]
pub async fn get_client_folder(year: &str, version: &str) -> Result<String> {
    let path = convert_err(player::get_client_folder(year, version))?;

    let Some(folder) = path.to_str() else {
        return Err("Couldnt get client folder".into());
    };
    Ok(folder.into())
}
