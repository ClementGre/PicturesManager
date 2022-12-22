use std::sync::{Mutex, MutexGuard};

use tauri::AppHandle;

use crate::header::window::new_window;

use super::gallery_data::Gallery;

#[derive(Debug, Default)]
pub struct WindowsGalleriesState {
    galleries: Mutex<Vec<WindowGallery>>,
}
#[derive(Debug, Default)]
pub struct WindowGallery {
    window_label: String,
    path: String,
    gallery: Gallery,
}

impl WindowsGalleriesState {
    fn get_galleries(&self) -> MutexGuard<'_, Vec<WindowGallery>> {
        self.galleries
            .lock()
            .unwrap()
    }

    fn get_new_unique_label(&self) -> String {
        let mut label = String::from("gallery-0");
        let mut i = 0;
        while self.galleries.lock().unwrap().iter().any(|gallery| gallery.window_label == label) {
            i += 1;
            label = format!("gallery-{}", i);
        }
        label
    }

    fn add_and_get_label(&self, path: String) -> String {
        let label = self.get_new_unique_label();
        self.galleries.lock().unwrap().push(WindowGallery {
            window_label: label.clone(),
            path,
            gallery: Gallery::default()
        });
        label
    }

    pub fn open_from_path(&self, app_handle: &mut AppHandle, path: String){
        let label = self.add_and_get_label(path.clone());
        new_window(app_handle, label, path);
    }
}

impl WindowGallery {
    #[allow(dead_code)]
    pub fn get_gallery(&self) -> &Gallery {
        &self.gallery
    }
    #[allow(dead_code)]
    pub fn get_gallery_mut(&mut self) -> &mut Gallery {
        &mut self.gallery
    }
    #[allow(dead_code)]
    pub fn get_path(&self) -> &String {
        &self.path
    }
    #[allow(dead_code)]
    pub fn get_label(&self) -> &String {
        &self.window_label
    }
}