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
