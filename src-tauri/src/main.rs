#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod header;
#[cfg(target_os = "macos")]
use header::macos::WindowExt;
#[cfg(target_os = "macos")]
use header::menubar::setup_menubar;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut builder = tauri::Builder::default().setup(|app| {
        let win = app.get_window("main").unwrap();
        #[cfg(target_os = "macos")]
        win.set_transparent_titlebar(header::macos::ToolbarThickness::Thick);
        #[cfg(not(target_os = "macos"))]
        {
            win.set_decorations(false)
                .expect("Unsupported platform! (Removing decorations)");
            use window_shadows::set_shadow;
            set_shadow(&win, true).expect("Unsupported platform! (Applying window decorations)");
        }

        Ok(())
    });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(setup_menubar(
            String::from("Pictures Manager"),
        ));
    }

    builder
        .on_menu_event(|event| {
            // Only custom menus
            println!("MenuEvent: {}", event.menu_item_id());
            match event.menu_item_id() {
                "close" => {}
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
