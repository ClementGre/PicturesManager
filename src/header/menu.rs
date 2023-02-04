use yew::Properties;

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
            id: "sepatator_".to_string() + count.to_string().as_str(),
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

pub fn get_menus() -> Vec<MenuItem> {
    let mut menus = Vec::new();
    menus.push(MenuItem::new_menu(
        "file".to_string(),
        "_Fichier".to_string(),
        vec![
            MenuItem::new_item_acc("open_gallery".to_string(), "_Ouvrir une gallerie".to_string(), "Ctrl+O".to_string()),
            MenuItem::new_menu("open_recent_gallery".to_string(), "Ouvrir une gallerie _récente".to_string(), vec![
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
            MenuItem::new_item_acc("new_gallery".to_string(), "_Nouvelle gallerie".to_string(), "Ctrl+N".to_string()),
            MenuItem::new_separator(0),
            MenuItem::new_item_acc("close_window".to_string(), "_Fermer la fenêtre".to_string(), "Ctrl+W".to_string()),
            MenuItem::new_item_acc("quit".to_string(), "_Quitter".to_string(), "Ctrl+Q".to_string()),
            MenuItem::new_separator(1),
            MenuItem::new_item_acc("settings".to_string(), "_Paramètres".to_string(), "Ctrl+Alt+S".to_string()),
            MenuItem::new_item("about".to_string(), "_À propos".to_string()),
        ],
    ));
    menus.push(MenuItem::new_menu(
        "edit".to_string(),
        "_Édition".to_string(),
        vec![
            MenuItem::new_item_acc("undo".to_string(), "_Annuler".to_string(), "Ctrl+Z".to_string()),
            MenuItem::new_item_acc("redo".to_string(), "_Rétablir".to_string(), "Ctrl+Shift+Z".to_string()),
            MenuItem::new_separator(3),
            MenuItem::new_item_acc("cut".to_string(), "_Couper".to_string(), "Ctrl+X".to_string()),
            MenuItem::new_item_acc("copy".to_string(), "Co_pier".to_string(), "Ctrl+C".to_string()),
            MenuItem::new_item_acc("paste".to_string(), "Co_ller".to_string(), "Ctrl+V".to_string()),
            MenuItem::new_item_acc("select_all".to_string(), "Tout _sélectionner".to_string(), "Ctrl+A".to_string()),
        ],
    ));
    menus.push(MenuItem::new_menu(
        "tools".to_string(),
        "_Outils".to_string(),
        vec![
            MenuItem::new_item("update_Gallery".to_string(), "_Actualiser la galerie".to_string()),
            MenuItem::new_item("edit_exif".to_string(), "_Corriger les données Exif".to_string()),
        ],
    ));
    menus
}
