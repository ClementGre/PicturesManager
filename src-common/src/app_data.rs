use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq, Store)]
pub struct Settings {
    pub theme: Theme,
    pub language: Option<String>,
}


#[derive(Deserialize, Serialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}