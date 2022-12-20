
#[tauri::command]
pub fn get_os() -> u16 {
    
    if cfg!(target_os = "windows") {
        return 0;
    }else if cfg!(target_os = "macos") {
        return 1;
    }else{
        return 2;
    }

}