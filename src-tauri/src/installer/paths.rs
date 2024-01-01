use std::fmt::{ Display, self };
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use dirs;

use super::APP_NAME;

#[derive(Debug)]
pub struct ErrNoPath {
    path: String,
}

impl Display for ErrNoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Dose not exist", self.path)
    }
}

impl Error for ErrNoPath {}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn dir_option_wrapper(dir: Option<PathBuf>) -> Result<PathBuf> {
    let Some(path) = dirs::data_local_dir() else {
        let path = ErrNoPath { path: format!("{:?}", dir) };
        return Err(path.into());
    };

    return dir_wrapper(path);
}

fn dir_wrapper(path: PathBuf) -> Result<PathBuf> {
    if !path.exists() {
        fs::create_dir(&path)?;
    }

    return Ok(path);
}

pub fn get_app_folder() -> Result<PathBuf> {
    let dir = dir_option_wrapper(dirs::data_local_dir())?;
    let path = dir_wrapper(dir.join(APP_NAME))?;
    Ok(path)
}

pub fn get_downloads_folder() -> Result<PathBuf> {
    dir_wrapper(get_app_folder()?.join("Downloads"))
}

pub fn get_clients_folder() -> Result<PathBuf> {
    let app_folder = get_app_folder()?;
    let clients = app_folder.join("clients");

    dir_wrapper(clients)
}

pub fn get_studio_folder() -> Result<PathBuf> {
    let app_folder = get_app_folder()?;
    let studios = app_folder.join("studios");

    dir_wrapper(studios)
}

pub fn get_year_folder<T: AsRef<str>>(year: T) -> Result<PathBuf> {
    let string: &str = year.as_ref();
    return dir_wrapper(get_clients_folder()?.join(string));
}

pub fn get_version_folder<T: AsRef<str>, Y: AsRef<str>>(year: T, version: Y) -> Result<PathBuf> {
    let year_folder = get_year_folder(year)?;
    let version: &str = version.as_ref();
    return dir_wrapper(year_folder.join(version));
}
