use std::{ error::Error, env, fmt::Display, collections::HashMap, path::PathBuf };
use serde::{ Serialize, Deserialize };
use tokio::fs;

/*
    The .desktop files arent actually ini they are there own thing but ini seems to work just fine
    Also the files are now compiled from structs instead of using the format! macro
*/

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "Desktop Entry")]
    desktop: Desktop,
}

#[derive(Serialize, Deserialize)]
pub struct Desktop {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Exec")]
    exec: String,
    #[serde(rename = "Terminal")]
    terminal: String,
    #[serde(rename = "Type")]
    app_type: String,
    #[serde(rename = "MimeType")]
    mime_type: String,
    #[serde(rename = "Icon")]
    icon: String,
    #[serde(rename = "StartupWMClass")]
    startup_wm_class: String,
    #[serde(rename = "Categories")]
    categories: String,
    #[serde(rename = "Comment")]
    comment: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mimetypes {
    #[serde(rename = "Default Applications")]
    default_apps: HashMap<String, String>,
}

#[derive(Debug)]
pub struct CouldntLocateExe;

impl Display for CouldntLocateExe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldnt locate the binary")
    }
}

impl Error for CouldntLocateExe {}

#[derive(Debug)]
pub struct CouldntGetFolder;

impl Display for CouldntGetFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "an xdg dir dosent exist")
    }
}

impl Error for CouldntGetFolder {}

#[derive(Debug)]
pub struct CouldntFindDefault;

impl Display for CouldntFindDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldnt find [Default Applications] in mimetypes")
    }
}

impl Error for CouldntFindDefault {}

pub fn generate_desktop_str() -> Result<String> {
    let exe_path = env::current_exe()?;
    let Some(location) = exe_path.to_str() else {
        return Err(CouldntLocateExe.into());
    };

    let desktop = Entry {
        desktop: Desktop {
            name: "SYNTAX".into(),
            exec: format!("{} %u", location),
            terminal: "false".into(),
            app_type: "Application".into(),
            mime_type: "x-scheme-handler/syntax-player;".into(),
            icon: format!("{}", location),
            startup_wm_class: "SyntaxLauncher".into(),
            categories: "Game;".into(),
            comment: "Syntax Launcher".into(),
        },
    };

    Ok(serde_ini::to_string(&desktop)?)
}

pub fn generate_mimetypes_str() -> Result<String> {
    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("x-scheme-handler/syntax-player".into(), "syntax-player.desktop".into());

    Ok(serde_ini::to_string(&values)?)
}

async fn generate_desktop() -> Result<()> {
    let desktop_content = generate_desktop_str()?;
    let Some(data_dir) = dirs::data_local_dir() else {
        return Err(CouldntGetFolder.into());
    };
    let desktop_file = data_dir.join("applications").join("syntax-player.desktop");

    fs::write(desktop_file, desktop_content).await?;
    Ok(())
}

async fn create_mimetypes(location: PathBuf) -> Result<()> {
    let content = format!("[Default Applications]\n{}", generate_mimetypes_str()?);
    fs::write(location, content).await?;
    Ok(())
}

async fn add_to_mimetypes(location: PathBuf) -> Result<()> {
    if !location.exists() {
        return create_mimetypes(location).await;
    }
    let content_future = fs::read(&location);
    let content = generate_mimetypes_str()?;
    let old_content = String::from_utf8(content_future.await?)?;

    if old_content.contains(&content) {
        return Ok(());
    }

    let new = old_content.replace(
        "[Default Applications]\n",
        &format!("[Default Applications]\n{}", content)
    );
    fs::write(location, new).await?;

    Ok(())
}

async fn generate_mimetypes() -> Result<()> {
    let Some(cfg_dir) = dirs::config_dir() else {
        return Err(CouldntGetFolder.into());
    };
    let Some(data_dir) = dirs::data_local_dir() else {
        return Err(CouldntGetFolder.into());
    };

    /* Start both of them so they can run at the same time */
    let future_one = add_to_mimetypes(cfg_dir.join("mimeapps.list"));
    let future_two = add_to_mimetypes(data_dir.join("mimeapps.list"));

    future_one.await?;
    future_two.await?;

    Ok(())
}

/* I DO NOT KNOW IF THIS WORKS I AM WRITING THIS ON WINDOWS BUT IN THEROY THIS SHOULD RUN */
pub async fn set_defaults() -> Result<()> {
    let future_one = generate_mimetypes();
    let future_two = generate_desktop();

    future_one.await?;
    future_two.await?;

    Ok(())
}
