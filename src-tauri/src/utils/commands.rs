use std::env;

use super::translator::TranslatorState;
use crate::{app_data::AppDataState, gallery::windows_galleries::WindowsGalleriesState};
use fluent::{FluentArgs, FluentValue};
use log::trace;
use tauri::Window;


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
        app_data.data().settings.language.clone().unwrap_or(String::from("Os defined")),
        galleries
            .get_galleries()
            .iter()
            .find(|gallery| gallery.get_label() == window.label())
            .unwrap()
            .get_path(),
        translator.tra("test", &args)
    )
}
