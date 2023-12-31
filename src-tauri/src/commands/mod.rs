use std::{ fmt::Display, env };

use crate::installer;
use serde::Serialize;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Serialize)]
pub struct BootstrapperInfo {
    compile_time: String,
    base_url: String,
    pkg_version: String,
}

#[tauri::command]
pub fn info() -> BootstrapperInfo {
    let compile_time: String = macros::compile_time!();
    let base_url: String = installer::BASE_URL.into();
    let pkg_version: String = env!("CARGO_PKG_VERSION").into();

    BootstrapperInfo {
        compile_time,
        base_url,
        pkg_version,
    }
}

fn convert_err<T, V: Display>(r: std::result::Result<T, V>) -> Result<T> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
pub async fn download_to_zip(version: &str, name: &str) -> Result<String> {
    let _ = convert_err(installer::download_to_zip(version, name).await)?;
    return Ok(name.to_string());
}

#[tauri::command]
pub async fn extract_zip(version: &str, name: &str) -> Result<String> {
    let _ = convert_err(installer::extract_zip(name, version).await)?;
    return Ok(name.to_string());
}

#[tauri::command]
pub async fn is_installed() -> Result<bool> {
    let latest_version = convert_err(installer::latest_version().await)?;
    let version_dir = convert_err(installer::get_version_dir(latest_version))?;

    let exe_path = version_dir.join("SyntaxPlayerBeta.exe");

    return Ok(exe_path.exists());
}

#[tauri::command]
pub async fn join_game(
    version: &str,
    client_year: &str,
    auth_ticket: &str,
    join_script: &str
) -> Result<()> {
    convert_err(installer::join_game(version, client_year, auth_ticket, join_script))?;
    Ok(())
}

#[tauri::command]
pub fn get_launch() -> Vec<String> {
    return env::args().collect::<Vec<String>>();
}

#[tauri::command]
pub async fn register_uri() -> Result<()> {
    convert_err(installer::uri::register_uri().await)?;
    Ok(())
}

#[tauri::command]
pub async fn generate_appsettings(version: &str) -> Result<()> {
    convert_err(installer::generate_appsettings(version).await)
}

#[tauri::command]
pub async fn create_directorys(version: &str) -> Result<()> {
    let _install_dir = convert_err(installer::get_installation_dir())?;
    let _download_dir = convert_err(installer::get_downloads_dir())?;
    let _versions_dir = convert_err(installer::get_versions_dir())?;
    let _current_version_dir = convert_err(installer::get_version_dir(version))?;

    convert_err(installer::create_extraction_dirs(&version).await)?;
    Ok(())
}

#[tauri::command]
pub async fn latest_version() -> Result<String> {
    convert_err(installer::latest_version().await)
}

#[tauri::command]
pub async fn get_targets() -> Vec<&'static str> {
    let mut targets = vec![];
    for [target, _] in installer::DOWNLOAD_FILES {
        targets.push(target);
    }
    return targets;
}
