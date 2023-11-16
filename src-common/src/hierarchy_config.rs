use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyConfig {
    pub nested_levels: Vec<Vec<HierarchyRule>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyRule {
    pub applied_to: Option<Vec<String>>, // List of parents group names/id or None if applied to all
    pub group_rules: Vec<HierarchyGroupRule>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyGroupRule {
    pub filters: Vec<HierarchyFilter>,
    pub groups: Vec<HierarchyGroup>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyFilter {}
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyGroup {}
