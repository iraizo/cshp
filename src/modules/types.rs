use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Depot {
    pub name: String,
    pub manifests: Vec<Manifest>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub repo: String,
    pub depot: u32,
    pub id: u32,
    pub pseudo_out: String,
    pub download_out: String,
    pub filter: String,
    pub main_manifest: String
}