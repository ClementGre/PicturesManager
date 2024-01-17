use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Store)]
pub struct Settings {
    pub theme: Theme,
    pub language: Option<String>,
    pub force_win_header: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: None,
            force_win_header: false,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}
