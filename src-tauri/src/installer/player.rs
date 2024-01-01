use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::{ paths, download_from_repo, download_and_extract, create_manifest_dirs };
use super::Result;

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

pub async fn get_client_manifest<T: AsRef<str>>(version: T) -> Result<HashMap<String, String>> {
    let version = version.as_ref();
    let bytes = download_from_repo(format!("data/manifest/{}.json", version)).await?;
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
    let client_path = get_client_folder(year, version)?;

    /* There was an issue when using .collect() */
    let mut values: Vec<&str> = vec![];
    for value in manifest.values() {
        values.push(value);
    }

    create_manifest_dirs(&client_path, values).await?;

    Ok(())
}
