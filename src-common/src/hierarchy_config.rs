use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyConfig {
    pub nested_levels: Vec<Vec<HierarchyGroupRule>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyGroupRule {
    pub filters: Vec<HierarchyFilter>,
    pub groups: Vec<HierarchyGroup>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub enum HierarchyFilterType {
    #[default]
    All,
    ParentDir(String),              // Regex
    Tags(Vec<String>, Vec<String>), // Include, Exclude
    DateRange(String, String),      // From, To
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyFilter {}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub enum HierarchyGroupType {
    #[default]
    None,
    DateInterval,            // Start date, Interval unit, Interval
    DateCluster(String),     // Cluster Id
    LocationCluster(String), // Cluster Id
    Tags(String),            // Tag group Id
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
#[serde(default)]
pub struct HierarchyGroup {}
