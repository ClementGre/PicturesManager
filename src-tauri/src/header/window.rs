#[cfg(target_os = "macos")]
use super::macos::{ToolbarThickness, WindowExt};
use tauri::{AppHandle, Window, Manager};
use tauri_plugin_window_state::{WindowExt, StateFlags, AppHandleExt};
use urlencoding::encode;

#[tauri::command]
pub fn close_window(window: Window, app: AppHandle) {
    save_windows_states(&app);
    let _ = window.close();
}

pub fn quit_app(app: &AppHandle){
    save_windows_states(&app);
    app.windows().iter().for_each(|window| window.1.close().unwrap());
    app.exit(0);
}

pub fn save_windows_states(app: &AppHandle) {
    let _ = app.save_window_state(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED | StateFlags::FULLSCREEN);
}

pub fn new_window(app: &AppHandle, label: String, gallery_path: String) {
    let window_builder = tauri::WindowBuilder::new(
        app,
        label,
        tauri::WindowUrl::App(format!("index.html?p={}", encode(&gallery_path.as_str())).into()),
    )
    .min_inner_size(500.0, 300.0)
    .inner_size(800.0, 500.0)
    .visible(false);

    let window;
    #[cfg(target_os = "macos")]
    {
        window = window_builder
            .hidden_title(true)
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .build()
            .unwrap();

        window.set_transparent_titlebar(ToolbarThickness::Thick);
    }
    #[cfg(not(target_os = "macos"))]
    {
        window = window_builder.build().unwrap();

        window
            .set_decorations(false)
            .expect("Unsupported platform! (Removing decorations)");
        use window_shadows::set_shadow;
        set_shadow(&window, true).expect("Unsupported platform! (Applying window decorations)");
    }

    let _ = window.restore_state(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED | StateFlags::FULLSCREEN);
    let _ = window.show();
    let _ = window.set_focus();

}
