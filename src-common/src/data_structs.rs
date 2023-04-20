use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    LIGHT,
    DARK,
    #[default]
    SYSTEM,
}