use std::rc::Rc;

use yew::Properties;

use crate::utils::translator::Translator;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub name: Option<String>, // None if separator
    pub accelerator: Option<String>,
    pub items: Option<Vec<MenuItem>>, // Some if menu
}

impl MenuItem {
    pub fn new_separator(count: u16) -> Self {
        Self {
            id: "separator_".to_string() + count.to_string().as_str(),
            name: None,
            accelerator: None,
            items: None,
        }
    }
    pub fn new_item(id: String, name: String) -> Self {
        Self {
            id,
            name: Some(name),
            accelerator: None,
            items: None,
        }
    }
    pub fn new_item_acc(id: String, name: String, accelerator: String) -> Self {
        Self {
            id,
            name: Some(name),
            accelerator: Some(accelerator),
            items: None,
        }
    }
    pub fn new_menu(id: String, name: String, items: Vec<MenuItem>) -> Self {
        Self {
            id,
            name: Some(name),
            accelerator: None,
            items: Some(items),
        }
    }
}

pub fn get_menus(t: &Rc<Translator>) -> Vec<MenuItem> {
    let mut menus = Vec::new();
    menus.push(MenuItem::new_menu(
        "file".to_string(),
        t.tr("menu-bar-file"),
        vec![
            MenuItem::new_item_acc("open_gallery".to_string(), t.tr("menu-bar-file-open-gallery"), "Ctrl+O".to_string()),
            MenuItem::new_menu("open_recent_gallery".to_string(), t.tr("menu-bar-file-recent-galleries").to_string(), vec![
                MenuItem::new_item("recent_gallery_1".to_string(), "Gallerie récente _1".to_string()),
                MenuItem::new_item("recent_gallery_2".to_string(), "Gallerie récente _2".to_string()),
                MenuItem::new_menu("recent_gallery_other_1".to_string(), "_Autre (1)".to_string(), vec![
                    MenuItem::new_item("recent_gallery_3".to_string(), "Gallerie récente _3".to_string()),
                    MenuItem::new_item("recent_gallery_4".to_string(), "Gallerie récente _4".to_string()),
                ]),
                MenuItem::new_item("recent_gallery_5".to_string(), "_Gallerie récente 5".to_string()),
                MenuItem::new_menu("recent_gallery_other_2".to_string(), "A_utre (2)".to_string(), vec![
                    MenuItem::new_item("recent_gallery_6".to_string(), "G_allerie récente 6".to_string()),
                    MenuItem::new_item("recent_gallery_7".to_string(), "Ga_llerie récente 7".to_string()),
                ]),
                MenuItem::new_item("recent_gallery_8".to_string(), "Gallerie r_écente 8".to_string()),
            ]),
            MenuItem::new_item_acc("new_gallery".to_string(), t.tr("menu-bar-file-new-gallery"), "Ctrl+N".to_string()),
            MenuItem::new_separator(0),
            MenuItem::new_item_acc("close_window".to_string(), t.tr("menu-bar-file-close-gallery"), "Ctrl+W".to_string()),
            MenuItem::new_item_acc("quit".to_string(), t.tr("menu-bar-file-quit"), "Ctrl+Q".to_string()),
            MenuItem::new_separator(1),
            MenuItem::new_item_acc("settings".to_string(), t.tr("menu-bar-file-settings"), "Ctrl+Alt+S".to_string()),
            MenuItem::new_item("about".to_string(), t.tr("menu-bar-file-about")),
        ],
    ));
    menus.push(MenuItem::new_menu(
        "edit".to_string(),
        t.tr("menu-bar-edit"),
        vec![
            MenuItem::new_item_acc("undo".to_string(), t.tr("menu-bar-edit-undo"), "Ctrl+Z".to_string()),
            MenuItem::new_item_acc("redo".to_string(), t.tr("menu-bar-edit-redo"), "Ctrl+Shift+Z".to_string()),
            MenuItem::new_separator(3),
            MenuItem::new_item_acc("cut".to_string(), t.tr("menu-bar-edit-cut"), "Ctrl+X".to_string()),
            MenuItem::new_item_acc("copy".to_string(), t.tr("menu-bar-edit-copy"), "Ctrl+C".to_string()),
            MenuItem::new_item_acc("paste".to_string(), t.tr("menu-bar-edit-paste"), "Ctrl+V".to_string()),
            MenuItem::new_item_acc("select_all".to_string(), t.tr("menu-bar-edit-select-all"), "Ctrl+A".to_string()),
        ],
    ));
    menus.push(MenuItem::new_menu(
        "tools".to_string(),
        t.tr("menu-bar-tools"),
        vec![
            MenuItem::new_item("update_Gallery".to_string(), t.tr("menu-bar-tools-update-gallery")),
            MenuItem::new_item("edit_exif".to_string(), t.tr("menu-bar-tools-edit-exif")),
        ],
    ));
    menus
}
