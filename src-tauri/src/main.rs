#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use log::info;
use tauri::{http::ResponseBuilder, Manager};
use tauri_plugin_window_state::StateFlags;
use url::Url;

use app_data::{AppData, AppDataState};
use gallery::windows_galleries::{get_gallery_path, WindowsGalleriesState};
use header::macos::WindowMacosExt;
#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;
use header::menubar::{menu_close_window, menu_quit, menu_update_gallery};
use utils::commands::{greet, open_devtools};
use utils::logger::{get_logger_plugin, log_from_front};
use utils::thumbnails::{gen_image_thumbnail, get_existing_thumbnail, get_image_dimensions};
use utils::translator::TranslatorState;

use crate::app_data::{get_settings, set_settings};
use crate::gallery::gallery_cache::{get_gallery_datas_cache, get_gallery_paths_cache};
use crate::gallery::gallery_data::{get_gallery_data, get_gallery_settings, set_gallery_data, set_gallery_settings};
use crate::gallery::windows_galleries::WindowGallery;
use crate::header::window::close_window;
use crate::utils::translator::{get_available_locales, get_system_locale, get_translation_file, Translator};

mod app_data;
mod gallery;
mod header;
mod utils;

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
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                close_window(event.window().clone(), event.window().app_handle());
            }
            tauri::WindowEvent::Destroyed => {
                info!("🚩 Window {} destroyed", event.window().label());
                let app_handle = event.window().app_handle();

                let galleries = app_handle.state::<WindowsGalleriesState>();
                galleries.on_close(event.window().label().into());

                if event.window().app_handle().windows().len() == 0 {
                    info!("🚩 No more windows, tauri will exit");
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
        .register_uri_scheme_protocol("reqimg", move |app, request| {
            let res_not_found = ResponseBuilder::new().status(404).body(Vec::new());

            let url = Url::parse(request.uri()).unwrap();

            let label = url.query_pairs().find(|(key, _)| key == "window").unwrap().1.to_string();
            let window = app.get_window(&label).expect("window not found");

            let galleries_state = app.state::<WindowsGalleriesState>();
            let galleries = galleries_state.get_galleries();
            let gallery = WindowGallery::get(&galleries, &window);

            if let Some(url::Host::Domain(action)) = url.host() {
                match action {
                    "get-thumbnail" => {
                        // The frontend must make sure the thumbnail exists before by calling the command gen_image_thumbnail.
                        // This is only for data transfer.

                        let id = url.query_pairs().find(|(key, _)| key == "id").unwrap().1.to_string();

                        if let Some(data) = get_existing_thumbnail(&gallery.path, &id) {
                            ResponseBuilder::new().mimetype("image/png").body(data)
                        } else {
                            info!("🖼️ Sending no thumbnail {}", id);
                            res_not_found
                        }
                    }
                    _ => res_not_found,
                }
            } else {
                res_not_found
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
            // Data
            get_settings,
            set_settings,
            // Menus
            menu_quit,
            menu_close_window,
            menu_update_gallery,
            // Gallery
            get_gallery_path,
            get_gallery_datas_cache,
            get_gallery_paths_cache,
            get_gallery_data,
            set_gallery_data,
            get_gallery_settings,
            set_gallery_settings,
            // Images
            gen_image_thumbnail,
            get_image_dimensions,
            // Other commands
            greet,
            open_devtools
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
