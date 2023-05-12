use std::env;

use super::translator::TranslatorState;
use crate::{app_data::AppDataState, gallery::windows_galleries::WindowsGalleriesState};
use fluent::{FluentArgs, FluentValue};
use log::trace;
use pm_common::data_structs::Theme;
use tauri::Window;

#[tauri::command]
pub fn get_theme_or_os(window: Window, app_data: tauri::State<AppDataState>) -> Theme {
    let data = app_data.data();
    let theme = data.get_settings().get_theme();
    if *theme == Theme::SYSTEM {
        return if window.theme().unwrap_or(tauri::Theme::Light) == tauri::Theme::Light {
            Theme::LIGHT
        } else {
            Theme::DARK
        };
    }
    return theme.clone();
}
#[tauri::command]
pub fn is_system_theme(app_data: tauri::State<AppDataState>) -> bool {
    return app_data.data().get_settings().get_theme() == &Theme::SYSTEM;
}

#[tauri::command]
pub fn greet(
    window: Window,
    _: tauri::AppHandle,
    app_data: tauri::State<AppDataState>,
    galleries: tauri::State<WindowsGalleriesState>,
    translator: tauri::State<TranslatorState>,
    name: &str,
) -> String {
    trace!("FROM TRACE !!! {:?}", env::var("CARGO_CFG_TARGET_OS"));

    let mut args = FluentArgs::new();
    args.set("name", FluentValue::from("John"));

    format!(
        "Hello, {}!  window_label = {}  settings_language = {}  gallery_path = {}, message = {}",
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
            .get_path(),
        translator.tra("test", &args)
    )
}
