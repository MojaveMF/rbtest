use std::collections::HashMap;

use super::{ paths, download_from_repo };
use super::Result;

pub fn is_installed<T: AsRef<str>>(year: T) -> bool {
    let year = year.as_ref();
    let Ok(folder) = paths::get_studio_folder() else {
        return false;
    };
    return folder.join(year).exists();
}

pub async fn get_available() -> Result<HashMap<String, String>> {
    let file = download_from_repo("data/studios.json").await?;
    let decoded: HashMap<String, String> = serde_json::from_slice(&file)?;

    Ok(decoded)
}
