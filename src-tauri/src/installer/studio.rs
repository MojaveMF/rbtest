use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::{ paths, download_from_repo, download_and_extract };
use super::Result;

pub fn get_studio_folder<T: AsRef<str>>(year: T) -> Result<PathBuf> {
    let dir = paths::get_studio_folder()?.join(year.as_ref());
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }

    return Ok(dir);
}

pub fn is_installed<T: AsRef<str>>(year: T) -> bool {
    let Ok(path) = get_studio_folder(year) else {
        return false;
    };
    return path.join("AppSettings.xml").exists();
}

pub async fn get_available() -> Result<HashMap<String, String>> {
    let file = download_from_repo("data/studios.json").await?;
    let decoded: HashMap<String, String> = serde_json::from_slice(&file)?;

    Ok(decoded)
}

pub async fn download_studio<V: AsRef<str>, U: AsRef<str>>(year: V, url: U) -> Result<()> {
    let studio_folder = get_studio_folder(year)?;
    download_and_extract(url, studio_folder).await?;

    Ok(())
}
