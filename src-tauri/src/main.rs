#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::http::ResponseBuilder;
use tauri::Manager;
mod header;
use header::window::{setup_app_window, window_close, window_maximize, window_minimize};
use log::{info, trace};
use std::env;
use std::fs::read;
mod logger;
use logger::{get_logger_plugin, log_from_front};

#[cfg(target_os = "macos")]
use header::macos::WindowExt;
#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;

#[tauri::command]
fn greet(name: &str) -> String {
    trace!("FROM TRACE !!! {:?}", env::var("CARGO_CFG_TARGET_OS"));
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut builder = tauri::Builder::default().setup(|app| {
        setup_app_window(app.get_window("main").unwrap());

        let local_window =
            tauri::WindowBuilder::new(app, "local", tauri::WindowUrl::App("index.html".into()))
                .hidden_title(true)
                .title_bar_style(tauri::TitleBarStyle::Overlay)
                .build();
        setup_app_window(local_window.unwrap());

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
        .register_uri_scheme_protocol("reqimg", move |_, request| {
            info!("🚩Request: {:?}", request);

            let res_not_img = ResponseBuilder::new().status(404).body(Vec::new());
            if request.method() != "GET" {
                return res_not_img;
            }
            let uri = request.uri();
            let start_pos = match uri.find("?path=") {
                Some(_pos) => _pos + 6,
                None => return res_not_img,
            };
            let end_pos = match uri.find("&") {
                Some(_pos) => _pos,
                None => return res_not_img,
            };
            let path: String = uri[start_pos..end_pos].to_string();
            info!("🚩Request: {}", path);

            let local_img = if let Ok(data) = read(path) {
                tauri::http::ResponseBuilder::new()
                    .mimetype(format!("image/{}", &"png").as_str())
                    .body(data)
            } else {
                res_not_img
            };
            local_img
        })
        .plugin(get_logger_plugin())
        .invoke_handler(tauri::generate_handler![
            greet,
            log_from_front,
            window_minimize,
            window_maximize,
            window_close
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn is_img_extension(extension: &str) -> bool {
//   let ex: [&str; 6] = ["png", "jpg", "jpeg", "gif", "bmp", "webp"];
//   ex.iter().any(|e| *e == extension.to_lowercase())
// }
