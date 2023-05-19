#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use app_data::{AppData, AppDataState};
use gallery::windows_galleries::WindowsGalleriesState;
use header::macos::WindowMacosExt;
use header::window::save_windows_states;
use tauri::{http::ResponseBuilder, Manager};
mod header;
use log::info;
use tauri_plugin_window_state::StateFlags;
use url::Url;
use utils::images_utils::{get_thumbnail, get_image_thumbnail};
use utils::translator::TranslatorState;
mod utils;
use utils::commands::greet;
use crate::app_data::{get_settings, set_settings};
use crate::gallery::windows_galleries::WindowGallery;
use utils::logger::{get_logger_plugin, log_from_front};
mod app_data;
mod gallery;
use header::menubar::{menu_quit, menu_close_window};

#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;

use crate::utils::translator::{get_available_locales, get_system_locale, get_translation_file, Translator};
use crate::gallery::gallery_cache::{update_gallery_cache, get_gallery_datas_cache, get_gallery_paths_cache};

fn main() {

    rexiv2::register_xmp_namespace("PicturesManagerClementGre", "PicturesManagerClementGre").unwrap();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default().setup(|app| {
        let data = app.state::<AppDataState>();
        *data.data() = AppData::load(&app.app_handle());

        let translator = app.state::<TranslatorState>();
        *translator.translator.lock().unwrap() = Some(Translator::new(&app, data.data().settings.language.clone()));

        let galleries = app.state::<WindowsGalleriesState>();

        galleries.open_from_path(&mut app.app_handle(), String::from("/Users/clement/Pictures/Gallery"));

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
            tauri::WindowEvent::Resized(_) => {
                #[cfg(target_os = "macos")]
                event.window().position_traffic_lights();
            }
            tauri::WindowEvent::ThemeChanged(_) => {
                #[cfg(target_os = "macos")]
                event.window().position_traffic_lights();
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
            // Gallery
            update_gallery_cache,
            get_gallery_datas_cache,
            get_gallery_paths_cache,
            get_image_thumbnail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
