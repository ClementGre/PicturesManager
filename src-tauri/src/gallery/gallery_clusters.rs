use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LocationClusters {
    pub name: String,
    pub clusters: Vec<LocationCluster>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LocationCluster {
    pub name: String,
    pub pictures: Vec<u32>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DatesClusters {
    pub name: String,
    pub clusters: Vec<DatesCluster>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DatesCluster {
    pub name: String,
    pub pictures: Vec<u32>,
}
