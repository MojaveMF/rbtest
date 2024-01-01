use std::{ error::Error, path::Path, fs::{ self, File }, io::Write };
use futures_util::StreamExt;
use rand::{ distributions::Alphanumeric, Rng };

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

pub async fn latest_version() -> Result<String> {
    Ok(reqwest::get(format!("https://{}/version", SETUP_URL)).await?.text().await?)
}

pub async fn download_file<U: AsRef<str>, L: AsRef<Path>>(url: U, location: L) -> Result<()> {
    let url = url.as_ref();
    let file = location.as_ref();
    let result = reqwest::get(url).await?;

    let mut file = fs::File::create(file)?;
    let mut stream = result.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }

    Ok(())
}

pub async fn extract_zip<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) -> Result<()> {
    zip_extract::extract(File::open(from)?, to.as_ref(), false)?;
    Ok(())
}

pub async fn download_and_extract<U: AsRef<str>, O: AsRef<Path>>(url: U, out: O) -> Result<()> {
    let download_url = url.as_ref();
    let file_name =
        rand::thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>() +
        ".zip";
    let output_file = paths::get_downloads_folder()?.join(file_name);

    download_file(download_url, &output_file).await?;
    extract_zip(output_file, out).await?;

    Ok(())
}

pub async fn create_manifest_dirs<L: AsRef<Path>>(location: L, paths: Vec<&str>) -> Result<()> {
    let location = location.as_ref().to_path_buf();
    for path in paths {
        let mut p = "./".to_string();
        for split in path.split("/") {
            p += split;
            let dir = location.join(&p);
            if !dir.exists() {
                fs::create_dir(location.join(&p))?;
            }
        }
    }

    Ok(())
}

pub async fn download_from_repo<T: AsRef<str>>(file: T) -> Result<Vec<u8>> {
    let file = file.as_ref();
    let target_file = format!(
        "https://raw.githubusercontent.com/{}/{}/{}",
        REPO_NAME,
        TARGET_BRANCH,
        file
    );

    Ok(reqwest::get(target_file).await?.bytes().await?.to_vec())
}
