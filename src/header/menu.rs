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
        "Fichier".to_string(),
        vec![
            MenuItem::new_item_acc(
                "open_gallery".to_string(),
                "Ouvrir une gallery".to_string(),
                "Ctrl+O".to_string(),
            ),
            MenuItem::new_item_acc(
                "new_gallery".to_string(),
                "Nouvelle gallerie".to_string(),
                "Ctrl+N".to_string(),
            ),
            MenuItem::new_separator(0),
            MenuItem::new_item_acc(
                "close_window".to_string(),
                "Fermer la fenêtre".to_string(),
                "Ctrl+W".to_string(),
            ),
            MenuItem::new_item_acc(
                "quit".to_string(),
                "Quitter".to_string(),
                "Ctrl+Q".to_string(),
            ),
            MenuItem::new_separator(1),
            MenuItem::new_item_acc(
                "settings".to_string(),
                "Paramètres".to_string(),
                "Ctrl+Alt+S".to_string(),
            ),
            MenuItem::new_item("about".to_string(), "À propos".to_string()),
        ],
    ));
    menus.push(MenuItem::new_menu(
        "edit".to_string(),
        "Édition".to_string(),
        vec![
            MenuItem::new_item_acc(
                "undo".to_string(),
                "Annuler".to_string(),
                "Ctrl+Z".to_string(),
            ),
            MenuItem::new_item_acc(
                "redo".to_string(),
                "Rétablir".to_string(),
                "Ctrl+Shift+Z".to_string(),
            ),
            MenuItem::new_separator(0),
            MenuItem::new_item_acc(
                "cut".to_string(),
                "Couper".to_string(),
                "Ctrl+X".to_string(),
            ),
            MenuItem::new_item_acc(
                "copy".to_string(),
                "Copier".to_string(),
                "Ctrl+C".to_string(),
            ),
            MenuItem::new_item_acc(
                "paste".to_string(),
                "Coller".to_string(),
                "Ctrl+V".to_string(),
            ),
            MenuItem::new_item_acc(
                "select_all".to_string(),
                "Select all".to_string(),
                "Ctrl+A".to_string(),
            ),
        ],
    ));
    menus.push(MenuItem::new_menu(
        "tools".to_string(),
        "Outils".to_string(),
        vec![
            MenuItem::new_item(
                "update_Gallery".to_string(),
                "Actualiser la galerie".to_string(),
            ),
            MenuItem::new_item(
                "edit_exif".to_string(),
                "Corriger les données Exif".to_string(),
            ),
        ],
    ));
    menus
}
