use std::error::Error;

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod register;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub async fn register_uri() -> Result<()> {
    return register::set_defaults().await;
}

pub fn create_studio_shortcuts(versions: Vec<&str>) -> Result<()> {
    register::create_studio_shortcuts(versions)
}
