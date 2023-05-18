use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationClusters {
    pub name: String,
    pub clusters: Vec<LocationCluster>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationCluster {
    pub name: String,
    pub pictures: Vec<u32>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatesClusters {
    pub name: String,
    pub clusters: Vec<DatesCluster>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatesCluster {
    pub name: String,
    pub pictures: Vec<u32>,
}