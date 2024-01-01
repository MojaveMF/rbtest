use std::collections::HashMap;
use std::fs;
use std::path::{ PathBuf, Path };

use serde::Serialize;

use crate::installer::launch_application;

use super::{ paths, download_from_repo, create_manifest_dirs, BASE_URL };
use super::Result;

#[derive(Debug, Serialize)]
struct AppSettings {
    #[serde(rename = "Settings")]
    pub settings: AppSettingsSettings,
}
#[derive(Debug, Serialize)]
struct AppSettingsSettings {
    #[serde(rename = "ContentFolder")]
    content_folder: String,
    #[serde(rename = "BaseUrl")]
    base_url: String,
}
/* 
    He returns !!!!
*/
pub async fn generate_appsettings<P: AsRef<Path>>(location: P) -> Result<()> {
    let location = location.as_ref();
    let settings = AppSettings {
        settings: AppSettingsSettings {
            content_folder: "content".into(),
            base_url: BASE_URL.into(),
        },
    };

    let encoded = serde_xml_rs::to_string(&settings)?;
    let app_settings_path = location.join("AppSettings.xml");

    fs::write(app_settings_path, encoded)?;

    Ok(())
}

pub fn get_client_folder<T: AsRef<str>, V: AsRef<str>>(year: T, version: V) -> Result<PathBuf> {
    let dir = paths::get_clients_folder()?.join(year.as_ref());
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    let dir = dir.join(version.as_ref());
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    return Ok(dir);
}

pub fn installed<T: AsRef<str>, V: AsRef<str>>(year: T, version: V) -> bool {
    let Ok(folder) = get_client_folder(year, version) else {
        return false;
    };

    return folder.join("AppSettings.xml").exists();
}

pub async fn get_client_manifest<T: AsRef<str>>(version: T) -> Result<HashMap<String, String>> {
    let version = version.as_ref();
    let bytes = download_from_repo(format!("data/manifest/{}.json", version)).await?;
    println!("{}", String::from_utf8(bytes.clone()).unwrap());
    let hashmap = serde_json::from_slice(&bytes)?;

    Ok(hashmap)
}

pub async fn get_valid_clients() -> Result<Vec<String>> {
    let bytes = download_from_repo("data/clients.json").await?;
    let vector = serde_json::from_slice(&bytes)?;

    Ok(vector)
}

pub async fn prepare_client<T: AsRef<str>, V: AsRef<str>>(
    year: T,
    version: V,
    manifest: HashMap<String, String>
) -> Result<()> {
    let year = year.as_ref();
    let client_path = get_client_folder(year, version)?;

    if year == "2016" {
        generate_appsettings(&client_path).await?;
    }

    /* There was an issue when using .collect() */
    let mut values: Vec<&str> = vec![];
    for value in manifest.values() {
        values.push(value);
    }

    create_manifest_dirs(&client_path, values).await?;

    Ok(())
}

pub async fn launch_client<T: AsRef<str>, V: AsRef<str>>(
    year: T,
    version: V,
    args: &[&str]
) -> Result<()> {
    let client_folder = get_client_folder(year, version)?;
    let mut player_executeable = client_folder.join("SyntaxPlayerBeta.exe");

    if !player_executeable.exists() {
        player_executeable = client_folder.join("RobloxPlayerBeta.exe");
    }

    launch_application(player_executeable, args)?;
    Ok(())
}
