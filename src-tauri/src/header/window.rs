use tauri::Window;

use super::macos::{ToolbarThickness, WindowExt};

#[tauri::command]
pub fn window_minimize(window: tauri::Window){
    let _ = window.minimize();
}

#[tauri::command]
pub fn window_maximize(window: tauri::Window){
    let maximized = window.is_maximized();
    if maximized.is_ok() && maximized.unwrap() {
        let _ = window.unmaximize();
    }else{
        let _ = window.maximize();
    }
    
}

#[tauri::command]
pub fn window_close(window: tauri::Window){
    let _ = window.close();
}

pub fn setup_app_window(window: Window){
    #[cfg(target_os = "macos")]
    {
        window.set_transparent_titlebar(ToolbarThickness::Thick);
    }
    #[cfg(not(target_os = "macos"))]
    {
        window.set_decorations(false)
            .expect("Unsupported platform! (Removing decorations)");
        use window_shadows::set_shadow;
        set_shadow(&window, true).expect("Unsupported platform! (Applying window decorations)");
    }
}
