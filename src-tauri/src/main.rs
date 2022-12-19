#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod header;
use log::trace;
use std::env;
use header::window::{window_minimize, window_maximize, window_close};
mod logger;
use logger::{get_logger_plugin, log_from_front};
mod os;
use os::get_os;

#[cfg(target_os = "macos")]
use header::macos::WindowExt;
#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    trace!("FROM TRACE !!! {:?}", env::var("CARGO_CFG_TARGET_OS"));
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut builder = tauri::Builder::default().setup(|app| {
        let win = app.get_window("main").unwrap();
        #[cfg(target_os = "macos")]
        win.set_transparent_titlebar(header::macos::ToolbarThickness::Thick);
        #[cfg(not(target_os = "macos"))]
        {
            win.set_decorations(false)
                .expect("Unsupported platform! (Removing decorations)");
            use window_shadows::set_shadow;
            set_shadow(&win, true).expect("Unsupported platform! (Applying window decorations)");
        }

        Ok(())
    });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(setup_menubar(String::from("Pictures Manager")));
    }

    builder
        .on_menu_event(|event| {
            // Only custom menus
            println!("MenuEvent: {}", event.menu_item_id());
            match event.menu_item_id() {
                "close" => {}
                _ => {}
            }
        })
        .plugin(get_logger_plugin())
        .invoke_handler(tauri::generate_handler![greet, log_from_front, get_os, window_minimize, window_maximize, window_close])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
