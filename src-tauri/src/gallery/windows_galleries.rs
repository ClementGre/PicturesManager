use std::sync::{Mutex, MutexGuard};

use log::info;
use tauri::{AppHandle, Window, Wry};

use crate::header::window::new_window;

use super::gallery_data::Gallery;

#[derive(Debug, Default)]
pub struct WindowsGalleriesState {
    pub galleries: Mutex<Vec<WindowGallery>>,
}

#[derive(Debug, Default)]
pub struct WindowGallery {
    pub window_label: String,
    pub path: String,
    pub gallery: Gallery,
}

impl WindowsGalleriesState {
    pub fn get_galleries(&self) -> MutexGuard<'_, Vec<WindowGallery>> {
        self.galleries.lock().unwrap()
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

    // Called in order to open a new gallery window
    pub fn open_from_path(&self, app_handle: &AppHandle<Wry>, path: String) {
        let label = self.get_new_unique_label();

        self.galleries.lock().unwrap().push(WindowGallery {
            window_label: label.clone(),
            path: path.clone(),
            gallery: Gallery::load(&path),
        });

        new_window(app_handle, label);
    }
    // Called when a gallery window is closed
    pub fn on_close(&self, label: String) {
        self.get_galleries().retain(|gallery| {
            if gallery.window_label != label {
                true
            } else {
                info!("Saving gallery data for window {}", label);
                gallery.gallery.save(&gallery.path);
                false
            }
        });
    }
}

impl WindowGallery {
    pub fn get<'a>(galleries: &'a MutexGuard<'_, Vec<WindowGallery>>, window: &Window<Wry>) -> &'a WindowGallery {
        galleries
            .iter()
            .find(|gallery| gallery.window_label == window.label())
            .expect("Can't find a matching gallery to the current window")
    }
    pub fn get_mut<'a>(galleries: &'a mut MutexGuard<'_, Vec<WindowGallery>>, window: &Window<Wry>) -> &'a mut WindowGallery {
        galleries
            .iter_mut()
            .find(|gallery| gallery.window_label == window.label())
            .expect("Can't find a matching gallery to the current window")
    }
}

#[tauri::command]
pub fn get_gallery_path(galleries: tauri::State<WindowsGalleriesState>, window: Window<Wry>) -> String {
    let galleries = galleries.get_galleries();
    WindowGallery::get(&galleries, &window).path.clone()
}
