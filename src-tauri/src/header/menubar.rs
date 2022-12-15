#[cfg(target_os = "macos")]
use tauri::AboutMetadata;
#[cfg(target_os = "macos")]
use tauri::{Menu, MenuItem, Submenu, CustomMenuItem};



#[cfg(target_os = "macos")]
pub fn setup_menubar(app_name: String) -> Menu {
    let mut menu = Menu::new();

    ////////// PLATFORM MENUS //////////

    {
        menu = menu.add_submenu(Submenu::new(
            app_name.clone(),
            Menu::new()
                .add_native_item(MenuItem::About(app_name, AboutMetadata::default()))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    let mut file_menu = Menu::new();
    file_menu = file_menu.add_native_item(MenuItem::CloseWindow);
    
    file_menu = file_menu.add_native_item(MenuItem::Quit);
    
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    
    edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);

    menu = menu.add_submenu(Submenu::new(
        "View",
        Menu::new().add_native_item(MenuItem::EnterFullScreen),
    ));
    

    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);

    window_menu = window_menu.add_native_item(MenuItem::Zoom);
    window_menu = window_menu.add_native_item(MenuItem::Separator);
    
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("Test", Menu::new().add_item(quit).add_item(close));
    menu = menu
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    menu
}
