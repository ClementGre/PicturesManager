use pm_common::data_structs::Theme;
use serde_wasm_bindgen;
use crate::app_data::AppDataState;

#[tauri::command]
pub fn get_theme(app_data: tauri::State<AppDataState>) -> Theme {
    app_data.data().get_settings().get_theme().clone()
}
