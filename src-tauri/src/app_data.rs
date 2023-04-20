use std::{fs::create_dir_all, fs::File, io::{BufWriter, BufReader}, sync::{Mutex, MutexGuard}};

use pm_common::data_structs::Theme;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle};

#[derive(Default)]
pub struct AppDataState {
    data: Mutex<AppData>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct AppData {
    settings: Settings,
    last_gallery: Option<String>
}

#[derive(Deserialize, Serialize, Default)]
pub struct Settings {
    theme: Theme,
    language: Option<String>,
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
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }
}
impl Settings {
    #[allow(dead_code)]
    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }
    #[allow(dead_code)]
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
    #[allow(dead_code)]
    pub fn get_language(&self) -> &Option<String> {
        &self.language
    }
    #[allow(dead_code)]
    pub fn set_language(&mut self, language: Option<String>) {
        self.language = language;
    }
}