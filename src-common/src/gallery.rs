use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct GalleryData {
    pub current_left_tab: u16,
    pub files_tab_selected_dir: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct GallerySettings {
    pub test: String,
}
