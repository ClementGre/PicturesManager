use std::{fs::create_dir_all, fs::File, io::{BufWriter, BufReader}, sync::{Mutex, MutexGuard}};

use pm_common::app_data::Settings;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Default)]
pub struct AppDataState {
    data: Mutex<AppData>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct AppData {
    pub settings: Settings,
    pub last_gallery: Option<String>
}

impl AppDataState {

    pub fn save(&self, app: &AppHandle) {
        let dir = app.path_resolver().app_data_dir().unwrap();
        let file = dir.join("app_data.json");

        create_dir_all(&dir).expect("Unable to create app data directory.");

        let file = File::create(&file).expect("Unable to create settings file");
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &(*self.data.lock().unwrap())).unwrap();
    }
    pub fn data(&self) -> MutexGuard<'_, AppData> {
        self.data.lock().unwrap()
    }
}
impl AppData {
    pub fn load(app: &AppHandle) -> Self {
        let dir = app.path_resolver().app_data_dir().unwrap();
        let file = dir.join("app_data.json");

        if file.exists() {
            let file = File::open(&file).expect("Unable to open settings file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("Unable to parse settings file")
        }else {
            AppData::default()
        }
    }
}

#[tauri::command]
pub fn get_settings(app_data: tauri::State<AppDataState>) -> Settings {
    app_data.data().settings.clone()
}

#[tauri::command]
pub fn set_settings(app: AppHandle, app_data: tauri::State<AppDataState>, settings: Settings) {
    app_data.data().settings = settings.clone();
    app_data.save(&app);
    app.emit_all("settings-changed", settings).unwrap();
}