use serde::{Deserialize, Serialize};

use crate::utils::utils::cmd_arg;

#[derive(Serialize, Deserialize)]
pub struct ContextMenu {
    pub items: Vec<MenuItem>,
}
#[derive(Serialize, Deserialize, Default)]
pub struct MenuItem {
    pub label: String,
    pub disabled: bool,
    pub checked: bool,
    pub event: String,
    pub payload: String,
    pub shortcut: String,
    pub icon: Option<MenuIcon>,
    pub sub_items: Vec<MenuItem>,
    pub is_separator: bool,
}
#[derive(Serialize, Deserialize, Default)]
pub struct MenuIcon {
    pub path: String,
}

impl ContextMenu {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }
    pub fn add_separator(&mut self) {
        self.items.push(MenuItem {
            is_separator: true,
            ..Default::default()
        });
    }
    pub fn show(&self) {
        cmd_arg("plugin:context_menu|show_context_menu", &self);
    }
}
