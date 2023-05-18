use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagGroup {
    pub name: String,
    pub multiple: bool,
    pub tags: HashMap<String, Tag>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tag {
    pub name: String,
    pub color: String,
    pub pictures: Vec<String>,
}