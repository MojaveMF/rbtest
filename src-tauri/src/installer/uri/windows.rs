use std::env;
use std::error::Error;
use std::fmt::Display;
use mslnk::ShellLink;
use winreg::RegKey;
use winreg::enums::*;
use mslnk;

use crate::installer::paths;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct CouldntLocateExe;

impl Display for CouldntLocateExe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldnt locate the binary")
    }
}

impl Error for CouldntLocateExe {}

pub async fn create_studio_shortcuts(versions: Vec<&str>) -> Result<()> {
    println!("Getting paths");
    let path = paths::shortcut_path()?;
    let exe_path = env::current_exe()?;
    let Some(target) = exe_path.to_str() else {
        return Err(CouldntLocateExe.into());
    };

    for year in versions {
        let target = format!("{}", target);
        let output_location = path.join(format!("Syntax Studio {}.lnk", year));
        if output_location.exists() {
            continue;
        }
        println!("{}", output_location.display());
        let mut sl = ShellLink::new(target)?;
        sl.set_arguments(Some(format!("--studio {}", year)));
        sl.create_lnk(output_location)?;
    }

    Ok(())
}

/* This function is wrongly a future but thats to keep compatability with the linux function */
pub async fn set_defaults() -> Result<()> {
    /* Modified from original src 
    /  Changes
    / [-] Uneeded function calls
    / [-] Bad use of tupples
    / [+] Error handling
    /
    */

    let exe_path = env::current_exe()?;
    let Some(current_exe_path) = exe_path.to_str() else {
        return Err(CouldntLocateExe.into());
    };

    let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);
    let hkey_classes_root: RegKey = hkey_current_user.open_subkey("Software\\Classes")?;
    let (hkey_syntax_player, _) = hkey_classes_root.create_subkey("syntax-player")?;
    let (hkey_syntax_player_shell, _) = hkey_syntax_player.create_subkey("shell")?;
    let (hkey_syntax_player_shell_open, _) = hkey_syntax_player_shell.create_subkey("open")?;
    let (hkey_syntax_player_shell_open_command, _) =
        hkey_syntax_player_shell_open.create_subkey("command")?;

    let (defaulticon, _) = hkey_syntax_player.create_subkey("DefaultIcon")?;
    hkey_syntax_player_shell_open_command.set_value(
        "",
        &format!("\"{}\" \"%1\"", current_exe_path)
    )?;
    defaulticon.set_value("", &format!("\"{}\",0", current_exe_path))?;
    hkey_syntax_player.set_value("", &format!("URL: Syntax Protocol"))?;
    hkey_syntax_player.set_value("URL Protocol", &"")?;

    Ok(())
}
