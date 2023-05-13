use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq)]
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