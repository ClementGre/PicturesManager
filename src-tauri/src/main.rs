#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app_data::{AppData, AppDataState};
use gallery::windows_galleries::WindowsGalleriesState;
use tauri::{http::ResponseBuilder, Manager, Window};
mod header;
use header::window::{window_close, window_maximize, window_minimize};
use log::{info, trace};
use std::env;
use std::fs::read;
use utils::translator::TranslatorState;
mod utils;
use utils::logger::{get_logger_plugin, log_from_front};
mod app_data;
mod gallery;

#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;
use header::menubar::menu_quit;

use crate::utils::translator::{get_language, Translator};

#[tauri::command]
fn greet(
    window: Window,
    _: tauri::AppHandle,
    app_data: tauri::State<AppDataState>,
    galleries: tauri::State<WindowsGalleriesState>,
    _: tauri::State<TranslatorState>,
    name: &str,
) -> String {
    trace!("FROM TRACE !!! {:?}", env::var("CARGO_CFG_TARGET_OS"));

    format!(
        "Hello, {}!  window_label = {}  settings_language = {}  gallery_path = {}",
        name,
        window.label(),
        app_data
            .data()
            .get_settings()
            .get_language()
            .clone()
            .unwrap_or(String::from("Os defined")),
        galleries
            .get_galleries()
            .iter()
            .find(|gallery| gallery.get_label() == window.label())
            .unwrap()
            .get_path()
    )
}



fn main() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default().setup(|app| {
        let data = app.state::<AppDataState>();
        *data.data() = AppData::load(&app.app_handle());

        let translator = app.state::<TranslatorState>();
        *translator.translator.lock().unwrap() = Some(Translator::new(
            data.data().get_settings().get_language().clone(),
        ));

        // let mut errors = vec![];
        // let bundle = translator.translator.lock().unwrap();
        // let bundle = &bundle.as_ref().unwrap().bundles;
        // info!("Test translation: {}", bundle.format_pattern(bundle.get_message("test").unwrap().value().unwrap(), None, &mut errors));

        let galleries = app.state::<WindowsGalleriesState>();

        galleries.open_from_path(
            &mut app.app_handle(),
            String::from("/Users/clement/Downloads/Gallery"),
        );

        Ok(())
    });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(setup_menubar(String::from("Pictures Manager")));
    }

    builder
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(_) => {}
            tauri::WindowEvent::Destroyed => {
                let app_handle = event.window().app_handle();

                let galleries = app_handle.state::<WindowsGalleriesState>();
                galleries.on_close(event.window().label().into());

                if event.window().app_handle().windows().len() == 0 {
                    info!("🚩No more windows, exiting");
                    event
                        .window()
                        .app_handle()
                        .state::<AppDataState>()
                        .save(&event.window().app_handle());
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
        .manage(TranslatorState::default())
        .manage(AppDataState::default())
        .manage(WindowsGalleriesState::default())
        .plugin(get_logger_plugin())
        .plugin(
            tauri_plugin_window_state::Builder::default()
            .with_denylist(&vec!["settings"])
                .with_show_mode(tauri_plugin_window_state::ShowMode::Always)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            log_from_front,
            window_minimize,
            window_maximize,
            window_close,
            get_language,
            menu_quit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn is_img_extension(extension: &str) -> bool {
//   let ex: [&str; 6] = ["png", "jpg", "jpeg", "gif", "bmp", "webp"];
//   ex.iter().any(|e| *e == extension.to_lowercase())
// }
