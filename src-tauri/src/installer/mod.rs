use std::fs as sfs;
use std::process::{ Command, exit };
use std::{ error::Error, path::PathBuf };
use std::fmt::Display;
use reqwest::{ get, ClientBuilder };
use futures_util::StreamExt;
use serde::Serialize;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub mod uri;

pub const SETUP_URL: &str = "setup.syntax.eco";
pub const BASE_URL: &str = "www.syntax.eco";

/* DOWNLOADS AND EXTRACTION PATHS */
pub const DOWNLOAD_FILES: [[&str; 2]; 19] = [
    /* Root */
    ["SyntaxApp.zip", "./"],
    ["NPSyntaxProxy.zip", "./"],
    ["SyntaxProxy.zip", "./"],
    ["Libraries.zip", "./"],
    ["redist.zip", "./"],
    /* Content */
    ["content-fonts.zip", "./content/fonts"],
    ["content-music.zip", "./content/music"],
    ["content-particles.zip", "./content/particles"],
    ["content-sky.zip", "./content/sky"],
    ["content-sounds.zip", "./content/sounds"],
    ["content-textures.zip", "./content/textures"],
    ["content-textures2.zip", "./content/textures"],
    ["content-scripts.zip", "./content/scripts"],
    /* Shaders */
    ["shaders.zip", "./shaders"],
    /* PlatformContent */
    ["content-terrain.zip", "./PlatformContent/pc/terrain"],
    ["content-textures3.zip", "./PlatformContent/pc/textures"],
    /* Clients */
    ["2018client.zip", "./Client2018"],
    ["2020client.zip", "./Client2020"],
    ["2014client.zip", "./Client2014"],
];

/* ERROR TYPES */

#[derive(Debug)]
struct CouldntGetFolder;

impl Display for CouldntGetFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldnt find folder")
    }
}

impl Error for CouldntGetFolder {}

#[derive(Debug)]
struct DosentExist;

impl Display for DosentExist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data dosent exist")
    }
}

impl Error for DosentExist {}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[cfg(target_os = "windows")]
fn launch_client(client_path: PathBuf, auth_ticket: &str, joinscript: &str) -> Result<()> {
    let mut cmd = Command::new(client_path);
    cmd.args([
        "--play",
        "--authenticationUrl",
        &format!("https://{}/Login/Negotiate.ashx", BASE_URL),
        "--joinScriptUrl",
        joinscript,
        "--authenticationTicket",
        auth_ticket,
    ]);
    cmd.spawn()?;
    //exit(0);
    Ok(())
}

#[cfg(target_os = "linux")]
fn launch_client(client_path: PathBuf, auth_ticket: &str, joinscript: &str) -> Result<()> {
    let Some(client_path) = client_path.to_str() else {
        return Err(DosentExist.into());
    };

    let mut cmd = Command::new("wine");
    cmd.args([
        client_path,
        "--play",
        "--authenticationUrl",
        &format!("https://{}/Login/Negotiate.ashx", BASE_URL),
        "--authenticationTicket",
        auth_ticket,
        "--joinScriptUrl",
        joinscript,
    ]);
    cmd.spawn()?;
    exit(0);
}

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

pub async fn generate_appsettings(version: &str) -> Result<()> {
    let settings = AppSettings {
        settings: AppSettingsSettings {
            content_folder: "content".into(),
            base_url: BASE_URL.into(),
        },
    };

    let encoded = serde_xml_rs::to_string(&settings)?;
    let app_settings_path = get_version_dir(version)?.join("AppSettings.xml");

    fs::write(app_settings_path, encoded).await?;

    Ok(())
}

pub fn join_game(
    version: &str,
    client_year: &str,
    auth_ticket: &str,
    joinscript: &str
) -> Result<()> {
    let version_location = get_version_dir(version)?;
    let client_location = (
        match client_year {
            "2016" => { version_location }
            _ => { version_location.join(format!("Client{}", client_year)) }
        }
    ).join("SyntaxPlayerBeta.exe");

    if !client_location.exists() {
        return Err(DosentExist.into());
    }

    launch_client(client_location, auth_ticket, joinscript)?;
    Ok(())
}

/// Makes a request to the setup url and returns the response as a vector
/// ```
/// let bin = request_setup("file.zip")?;
/// ```
pub async fn request_setup<T: Into<String>>(endpoint: T) -> Result<Vec<u8>> {
    let endpoint: String = endpoint.into();
    let request_url = format!("http://{}/{}", SETUP_URL, endpoint);

    return Ok(get(request_url).await?.bytes().await?.to_vec());
}

/// Takes a list of bytes and a [`String`] and extracts it
/// Example
/// ```
/// let zipped = request_api("SyntaxApp.zip")?;
/// extract_bin(&zipped,"./")?;
/// ```
///async fn extract_bin<P: Into<String>>(File: &[u8], Path: P) -> Result<()> {}

pub async fn latest_version() -> Result<String> {
    return Ok(String::from_utf8(request_setup("version").await?)?);
}

/* UTILITY FUNCTIONS */

/* dirs wrappers to get target folders easily */
pub fn get_installation_dir() -> Result<PathBuf> {
    let Some(data_local_dir) = dirs::data_local_dir() else {
        return Err(CouldntGetFolder.into());
    };
    return Ok(data_local_dir.join("Syntax"));
}

pub fn get_versions_dir() -> Result<PathBuf> {
    let dir = get_installation_dir()?.join("Versions");
    if !dir.exists() {
        sfs::create_dir_all(&dir)?;
    }
    return Ok(dir);
}

pub fn get_downloads_dir() -> Result<PathBuf> {
    let dir = get_installation_dir()?.join("Downloads");
    if !dir.exists() {
        sfs::create_dir_all(&dir)?;
    }
    return Ok(dir);
}

pub fn get_version_dir<T: Into<String>>(version: T) -> Result<PathBuf> {
    let dir = get_versions_dir()?.join(version.into());
    if !dir.exists() {
        sfs::create_dir_all(&dir)?;
    }
    return Ok(dir);
}

fn get_zip_path<T: Into<String>>(name: T) -> Result<PathBuf> {
    return Ok(get_downloads_dir()?.join(name.into()));
}

async fn create_dir_deep(version: &str, path: &str) -> Result<()> {
    let sections = path.split("/").collect::<Vec<&str>>();

    let mut p = get_version_dir(version)?;

    for section in sections {
        p = p.join(section);
        if !p.exists() {
            fs::create_dir(&p).await?;
        }
    }

    Ok(())
}

pub async fn create_extraction_dirs<V: AsRef<str>>(version: V) -> Result<()> {
    let version = version.as_ref();
    let version_path = get_version_dir(version)?;

    for [_, path] in DOWNLOAD_FILES {
        let p = version_path.join(path);
        if p.exists() {
            continue;
        }
        create_dir_deep(version, path).await?;
    }

    Ok(())
}

fn name_to_path<T: AsRef<str>>(name: T) -> Result<PathBuf> {
    let name = name.as_ref();
    for [file, path] in DOWNLOAD_FILES {
        if file == name {
            return Ok(PathBuf::from(path));
        }
    }
    return Err(CouldntGetFolder.into());
}

pub async fn extract_zip<V: Into<String>, T: Into<String>>(archive: V, version: T) -> Result<()> {
    let version_path = get_version_dir(version)?;
    let archive_name = archive.into();
    let archive_path = get_zip_path(&archive_name)?;
    let extract_location = version_path.join(name_to_path(&archive_name)?);

    if !archive_path.exists() {
        return Err(DosentExist.into());
    }

    println!("Extracting");
    zip_extract::extract(sfs::File::open(archive_path)?, &extract_location, false)?;
    println!("Extracted");

    Ok(())
}

pub async fn download_to_zip<V: Into<String>, T: Into<String>>(version: V, file: T) -> Result<()> {
    let file = file.into();
    let request_url = format!("http://{}/{}-{}", SETUP_URL, version.into(), file);
    let client = ClientBuilder::default().build()?;

    let file_location = get_zip_path(file)?;

    let response_future = client.get(request_url).send();
    let file_future = fs::File::create(file_location);

    let response = response_future.await?;
    let mut file = file_future.await?;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    return Ok(());
}
