use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct GalleryData {
    pub last_left_tab: u16,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct GallerySettings {
    pub test: String,
}
