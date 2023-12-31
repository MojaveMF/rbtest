// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ fmt::Display, error::Error };
use tauri::Manager;

mod installer;
mod commands;

#[derive(Debug)]
pub struct FailedInit;

impl Display for FailedInit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to init the ui")
    }
}

impl Error for FailedInit {}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                commands::info,
                commands::get_targets,
                commands::latest_version,
                commands::create_directorys,
                commands::register_uri,
                commands::download_to_zip,
                commands::extract_zip,
                commands::is_installed,
                commands::get_launch,
                commands::join_game,
                commands::generate_appsettings
            ]
        )
        .setup(|app| {
            let Some(window) = app.get_window("SYNTAX") else {
                return Err(FailedInit.into());
            };

            /* Focus and center */
            window.set_focus()?;
            window.center()?;
            window.set_always_on_top(true)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
