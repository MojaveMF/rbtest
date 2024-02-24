use std::{ error::Error, path::Path, fs::{ self, File }, io::Write, fmt::Display };
use futures_util::StreamExt;
use rand::{ distributions::Alphanumeric, Rng };
use std::process::Command;

pub mod uri;
pub mod paths;
pub mod studio;
pub mod player;

pub const APP_NAME: &str = "RbTest";
pub const BASE_URL: &str = "www.rbtest.org";
pub const SETUP_URL: &str = "setup.rbtest.org";

pub const REPO_NAME: &str = "MojaveMF/rbtest";

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
        let dir = location.join(path);

        if !dir.exists() {
            fs::create_dir_all(dir)?;
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

#[derive(Debug)]
pub struct CouldntLocateExe;

impl Display for CouldntLocateExe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldnt locate the binary")
    }
}

impl Error for CouldntLocateExe {}

#[cfg(target_os = "windows")]
pub fn launch_application<P: AsRef<Path>>(path: P, args: &[&str]) -> Result<()> {
    let path = path.as_ref();
    let mut cmd = Command::new(path);

    cmd.args(args);
    cmd.spawn()?;

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn launch_application<P: AsRef<Path>>(path: P, args: &[&str]) -> Result<()> {
    let path = path.as_ref();
    let Some(path_string) = path.to_str() else {
        return Err(CouldntLocateExe.into());
    };

    let mut cmd = Command::new("wine");
    cmd.arg(path_string);
    cmd.args(args);
    cmd.spawn()?;

    Ok(())
}
