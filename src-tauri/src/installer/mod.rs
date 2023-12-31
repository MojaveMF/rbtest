use std::error::Error;

pub mod uri;
pub mod paths;
pub mod studio;
pub mod player;

pub const APP_NAME: &str = "Syntax";
pub const BASE_URL: &str = "www.syntax.eco";
pub const SETUP_URL: &str = "setup.syntax.eco";

pub const REPO_NAME: &str = "MojaveMF/syntax";

#[cfg(debug_assertions)]
pub const TARGET_BRANCH: &str = "main";

#[cfg(not(debug_assertions))]
pub const TARGET_BRANCH: &str = "release";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub async fn download_from_repo<T: AsRef<str>>(file: T) -> Result<Vec<u8>> {
    let mut file = file.as_ref();
    let target_file = format!(
        "https://raw.githubusercontent.com/{}/{}/{}",
        REPO_NAME,
        TARGET_BRANCH,
        file
    );

    Ok(reqwest::get(target_file).await?.bytes().await?.to_vec())
}
