use std::{ error::Error, env, fmt::Display, collections::HashMap, path::PathBuf, process::Command };
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

pub fn generate_desktop_str(arguments: &[&str]) -> Result<String> {
    let exe_path = env::current_exe()?;
    let Some(location) = exe_path.to_str() else {
        return Err(CouldntLocateExe.into());
    };

    let desktop = Entry {
        desktop: Desktop {
            name: "SYNTAX".into(),
            exec: format!("{} {} %u", location, arguments.join(" ")),
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

async fn generate_uri<P: AsRef<str>, U: AsRef<str>>(desktop_file_name: P, uri: U) -> Result<()> {
    let uri = uri.as_ref();
    let name = desktop_file_name.as_ref();
    let generated_uri: String = format!("x-scheme-handler/{}", uri);
    let mut cmd = Command::new("xdg-mime");
    cmd.args(["default", name, &generated_uri]);

    cmd.spawn()?.wait()?;

    Ok(())
}

async fn generate_desktop<T: AsRef<str>>(
    name: T,
    arguments: &[&str],
    uri: Option<&str>
) -> Result<()> {
    let desktop_content = generate_desktop_str(arguments)?;
    let Some(data_dir) = dirs::data_local_dir() else {
        return Err(CouldntGetFolder.into());
    };
    let name = format!("{}.desktop", name.as_ref());
    let desktop_file = data_dir.join("applications").join(&name);
    if !desktop_file.exists() {
        fs::write(desktop_file, desktop_content).await?;

        if let Some(to_reg) = uri {
            generate_uri(name, to_reg).await?;
        }
    }
    Ok(())
}

pub async fn set_defaults() -> Result<()> {
    generate_desktop("syntax-desktop", &[], Some("syntax-player")).await
}

pub async fn create_studio_shortcuts(versions: Vec<&str>) -> Result<()> {
    for version in versions {
        generate_desktop(format!("syntax-studio-{}", version), &["--studio", version], None).await?;
    }

    Ok(())
}
