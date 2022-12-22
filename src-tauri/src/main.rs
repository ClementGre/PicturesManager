#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app_data::{AppData, AppDataState};
use gallery::windows_galleries::WindowsGalleriesState;
use sys_locale::get_locale;
use tauri::{http::ResponseBuilder, Manager, Window};
mod header;
use header::window::{new_window, window_close, window_maximize, window_minimize};
use log::{info, trace};
use std::fs::read;
use std::{env};
mod logger;
use logger::{get_logger_plugin, log_from_front};
mod app_data;
mod gallery;

#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;

#[tauri::command]
fn greet(
    window: Window,
    app_handle: tauri::AppHandle,
    state: tauri::State<AppDataState>,
    name: &str,
) -> String {
    trace!("FROM TRACE !!! {:?}", env::var("CARGO_CFG_TARGET_OS"));

    format!(
        "Hello, {}!  window = {}  gallery = {}  app = {}",
        name,
        window.label(),
        state.data().get_settings().get_language().clone().unwrap_or(String::from("Os defined")),
        app_handle.package_info().authors
    )
}

fn main() {
    let mut builder = tauri::Builder::default().setup(|app| {
        
        let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
        info!("🚩Locale: {}", locale);

        let data = app.state::<AppDataState>();
        *data.data() = AppData::load(&app.app_handle());

        info!(
            "🚩Locale in settings: {:?}",
            app.state::<AppDataState>()
                .data()
                .get_settings()
                .get_language().clone()
                .unwrap_or_else(|| String::from("OS defined"))
        );

        new_window(&app.app_handle(), "gallery-0".into(), String::from("/Users/clement/Downloads/Gallery"));
        new_window(&app.app_handle(), "gallery-1".into(), String::from("/Users/clement/Images/Gal&lery"));
        Ok(())
    });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(setup_menubar(String::from("Pictures Manager")));
    }

    builder
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(_) => {
                
            }
            tauri::WindowEvent::Destroyed => {
                if event.window().app_handle().windows().len() == 0 {
                    info!("🚩No more windows, exiting");
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
        .manage(AppDataState::default())
        .manage(WindowsGalleriesState::default())
        .plugin(get_logger_plugin())
        .plugin(tauri_plugin_window_state::Builder::default().build())
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
