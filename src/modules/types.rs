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
pub struct Api {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    
}