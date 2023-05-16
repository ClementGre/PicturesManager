#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use app_data::{AppData, AppDataState};
use gallery::windows_galleries::WindowsGalleriesState;
use header::window::{save_windows_states};
use tauri::{http::ResponseBuilder, Manager};
mod header;
use log::info;
use tauri_plugin_window_state::{StateFlags};
use std::fs::read;
use utils::translator::TranslatorState;
mod utils;
use utils::commands::{greet};
use crate::app_data::{get_settings, set_settings};
use utils::logger::{get_logger_plugin, log_from_front};
mod app_data;
mod gallery;

use header::menubar::{menu_quit, menu_close_window};
#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;

use crate::utils::translator::{get_available_locales, get_system_locale, get_translation_file, Translator};

fn main() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default().setup(|app| {
        let data = app.state::<AppDataState>();
        *data.data() = AppData::load(&app.app_handle());

        let translator = app.state::<TranslatorState>();
        *translator.translator.lock().unwrap() = Some(Translator::new(&app, data.data().settings.language.clone()));

        let galleries = app.state::<WindowsGalleriesState>();

        galleries.open_from_path(&mut app.app_handle(), String::from("/Users/clement/Downloads/Gallery"));

        Ok(())
    });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(setup_menubar(String::from("Pictures Manager")));
    }

    builder
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(_) => {}
            tauri::WindowEvent::CloseRequested { /* api, */ .. } => {
                save_windows_states(&event.window().app_handle());
            }
            tauri::WindowEvent::Destroyed => {
                let app_handle = event.window().app_handle();

                let galleries = app_handle.state::<WindowsGalleriesState>();
                galleries.on_close(event.window().label().into());

                

                if event.window().app_handle().windows().len() == 0 {
                    info!("🚩 No more windows, exiting");
                    
                    event.window().app_handle().state::<AppDataState>().save(&event.window().app_handle());
                }
            }
            _ => {}
        })
        .on_menu_event(|event| {
            // Only custom menus
            println!("MenuEvent: {}", event.menu_item_id());
            match event.menu_item_id() {
                "close" => {}
                _ => {}
            }
        })
        .register_uri_scheme_protocol("reqimg", move |_, request| {
            info!("🚩 Request: {:?}", request);

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
        .manage(TranslatorState::default())
        .manage(AppDataState::default())
        .manage(WindowsGalleriesState::default())
        .plugin(get_logger_plugin())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_denylist(&vec!["settings"])
                .with_state_flags(StateFlags::SIZE & StateFlags::POSITION & StateFlags::MAXIMIZED & StateFlags::FULLSCREEN)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            log_from_front,
            get_available_locales,
            get_system_locale,
            get_translation_file,
            greet,
            // Data
            get_settings,
            set_settings,
            // Menus
            menu_quit,
            menu_close_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn is_img_extension(extension: &str) -> bool {
//   let ex: [&str; 6] = ["png", "jpg", "jpeg", "gif", "bmp", "webp"];
//   ex.iter().any(|e| *e == extension.to_lowercase())
// }
