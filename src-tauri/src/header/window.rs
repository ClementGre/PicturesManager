use std::time::{SystemTime, UNIX_EPOCH};

use tauri::{App, Manager};
use urlencoding::encode;

use super::macos::{ToolbarThickness, WindowExt};

#[tauri::command]
pub fn window_minimize(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
pub fn window_maximize(window: tauri::Window) {
    let maximized = window.is_maximized();
    if maximized.is_ok() && maximized.unwrap() {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

#[tauri::command]
pub fn window_close(window: tauri::Window) {
    let _ = window.close();
}

pub fn new_window(app: &mut App, gallery_path: String) {
    let window = tauri::WindowBuilder::new(
        app,
        format!("local-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()),
        tauri::WindowUrl::App(format!("index.html?p={}", encode(&gallery_path.as_str())).into()),
    )
    .visible(false) // tauri-plugin-window-state is responsible for showing the window after the state is restored.
    .hidden_title(true)
    .title_bar_style(tauri::TitleBarStyle::Overlay)
    .min_inner_size(500.0, 300.0)
    .inner_size(800.0, 500.0)
    .build()
    .unwrap();

    window.manage(GalleryData { gallery_path });

    #[cfg(target_os = "macos")]
    {
        window.set_transparent_titlebar(ToolbarThickness::Thick);
    }
    #[cfg(not(target_os = "macos"))]
    {
        window
            .set_decorations(false)
            .expect("Unsupported platform! (Removing decorations)");
        use window_shadows::set_shadow;
        set_shadow(&window, true).expect("Unsupported platform! (Applying window decorations)");
    }
}

pub struct GalleryData {
    pub gallery_path: String,
}
