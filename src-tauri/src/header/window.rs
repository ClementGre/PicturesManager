use tauri::{AppHandle, Manager, Window, Wry};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

#[cfg(target_os = "macos")]
use super::macos::WindowMacosExt;

// Windows are always closed from frontend for saving ui configuration before closing
pub fn close_window(window: Window<Wry>, app: AppHandle<Wry>) {
    save_windows_states(&app);
    let _ = window.emit("tauri://close-requested", &()).expect("Failed to send window close request.");
}

pub fn quit_app(app: &AppHandle<Wry>) {
    save_windows_states(&app);
    app.windows().iter().for_each(|window| {
        window
            .1
            .emit("tauri://close-requested", &())
            .expect("Failed to send window close request.")
    });
}

pub fn save_windows_states(app: &AppHandle<Wry>) {
    let _ = app.save_window_state(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED | StateFlags::FULLSCREEN);
}

pub fn new_window(app: &AppHandle<Wry>, label: String) {
    let window_builder = tauri::WindowBuilder::new(app, label, tauri::WindowUrl::App(format!("index.html").into()))
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

        window.set_transparent_titlebar();
    }
    #[cfg(not(target_os = "macos"))]
    {
        window = window_builder.build().unwrap();

        window.set_decorations(false).expect("Unsupported platform! (Removing decorations)");
        use window_shadows::set_shadow;
        set_shadow(&window, true).expect("Unsupported platform! (Applying window decorations)");
    }

    let _ = window.restore_state(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED | StateFlags::FULLSCREEN);
    let _ = window.show();
    let _ = window.set_focus();
}
