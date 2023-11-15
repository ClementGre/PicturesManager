#[cfg(target_os = "macos")]
use tauri::AboutMetadata;
use tauri::{AppHandle, State, Window, Wry};
#[cfg(target_os = "macos")]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use crate::gallery::gallery_cache::update_gallery_cache;
use crate::gallery::windows_galleries::WindowsGalleriesState;
use crate::utils::translator::TranslatorState;

use super::window::{close_window, quit_app};

#[cfg(target_os = "macos")]
pub fn setup_menubar(app_name: String, t: &TranslatorState) -> Menu {
    let mut menu = Menu::new();

    ////////// PLATFORM MENUS //////////

    menu = menu.add_submenu(Submenu::new(
        app_name.clone(),
        Menu::new()
            .add_native_item(MenuItem::About(app_name, AboutMetadata::default()))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("settings".to_string(), "Settings...").accelerator("Cmd+,"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    ));

    let file_menu = Menu::new()
        .add_item(CustomMenuItem::new("open_gallery".to_string(), tr(t, "menu-bar-file-galleries")).accelerator("Cmd+O"))
        .add_item(CustomMenuItem::new(
            "open_recent_gallery".to_string(),
            tr(t, "menu-bar-file-open-recent-gallery"),
        ))
        .add_item(CustomMenuItem::new("new_gallery".to_string(), tr(t, "menu-bar-file-new-gallery")).accelerator("Cmd+N"))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::CloseWindow);

    let edit_menu = Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll);

    let tools_menu = Menu::new()
        .add_item(CustomMenuItem::new("update_gallery".to_string(), tr(t, "menu-bar-tools-update-gallery")))
        .add_item(CustomMenuItem::new("edit_exif".to_string(), tr(t, "menu-bar-tools-edit-exif")));

    let window_menu = Menu::new()
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Zoom)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::CloseWindow);

    ////////// OTHERS MENUS //////////

    menu.add_submenu(Submenu::new("File", file_menu))
        .add_submenu(Submenu::new("Edit", edit_menu))
        .add_submenu(Submenu::new("View", Menu::new().add_native_item(MenuItem::EnterFullScreen)))
        .add_submenu(Submenu::new("Tools", tools_menu))
        .add_submenu(Submenu::new("Window", window_menu))
}

fn tr(t: &TranslatorState, key: &str) -> String {
    t.tr(key).replace("_", "")
}

#[tauri::command]
pub fn menu_quit(app: AppHandle<Wry>) {
    quit_app(&app);
}
#[tauri::command]
pub fn menu_close_window(window: Window<Wry>, app: AppHandle<Wry>) {
    close_window(&window, &app);
}

#[tauri::command]
pub async fn menu_update_gallery(window: Window<Wry>, galleries_state: State<'_, WindowsGalleriesState>) -> Result<(), ()> {
    let data = update_gallery_cache(&window, galleries_state);
    window.emit("gallery-cache-changed", data).unwrap();
    Ok(())
}
