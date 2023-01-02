#[cfg(target_os = "macos")]
use super::macos::{ToolbarThickness, WindowExt};
use tauri::AppHandle;
use urlencoding::encode;

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

pub fn new_window(app_handle: &AppHandle, label: String, gallery_path: String) {
    let window_builder = tauri::WindowBuilder::new(
        app_handle,
        label,
        tauri::WindowUrl::App(format!("index.html?p={}", encode(&gallery_path.as_str())).into()),
    )
    .min_inner_size(500.0, 300.0)
    .inner_size(800.0, 500.0)
    .visible(false); // tauri-plugin-window-state is responsible for showing the window after the state is restored.

    #[cfg(target_os = "macos")]
    {
        let window = window_builder
            .hidden_title(true)
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .build()
            .unwrap();

        window.set_transparent_titlebar(ToolbarThickness::Thick);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let window = window_builder.build().unwrap();

        window
            .set_decorations(false)
            .expect("Unsupported platform! (Removing decorations)");
        use window_shadows::set_shadow;
        set_shadow(&window, true).expect("Unsupported platform! (Applying window decorations)");
    }
}
