use std::env;

use fluent::{FluentArgs, FluentValue};
use log::trace;
use tauri::{AppHandle, Window, Wry};

use crate::{app_data::AppDataState, gallery::windows_galleries::WindowsGalleriesState};

use super::translator::TranslatorState;

#[tauri::command]
pub fn greet(
    window: Window<Wry>,
    _: AppHandle<Wry>,
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
        app_data.data().settings.language.clone().unwrap_or(String::from("Os defined")),
        galleries
            .get_galleries()
            .iter()
            .find(|gallery| gallery.window_label == window.label())
            .unwrap()
            .path,
        translator.tra("test", &args)
    )
}

#[tauri::command]
pub fn open_devtools(window: Window<Wry>) {
    if window.is_devtools_open() {
        window.close_devtools();
    } else {
        window.open_devtools();
    }
}
