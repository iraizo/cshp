use std::process::Command;

use serde_json::Value;

use super::types::{Depot, Manifest};

pub struct Steamcmd {
    pub id: u32,
    pub depot: u32,
    pub dir: String
}

impl Steamcmd {
    pub fn new(id: u32, depot: u32, dir: String) -> Self {
        Self { id, depot, dir }
    }

    pub fn fetch_depot(&self) -> Depot {
        let resp: Value = match reqwest::blocking::get("https://api.steamcmd.net/v1/info/".to_owned() + &self.id.to_string()) {
            Ok(response) => {
                if response.status() != 200 {
                    panic!("Error trying to get depots, api returned status {:?}", response.status());
                }
                response.json().unwrap()
            },
            Err(err) => panic!("Error trying to send request {:?}", &err),
        };

        if resp["status"] != "success" {
            println!("{:?}", resp);
            std::process::exit(-1);
        }

        let mut depot = Depot {
            name: resp["data"][self.id.to_string()]["depots"][self.depot.to_string()]["name"].to_string(),
            manifests: vec![]
        };

        for (k, v) in resp["data"][self.id.to_string()]["depots"][self.depot.to_string()]["manifests"].as_object().unwrap() {
            depot.manifests.push(Manifest {
                name: k.to_string(),
                id: v.as_str().unwrap().to_string(),
            });
        }

        return depot;
    }


    /* some values are hardcoded since this is initally made for rust reversing */ 
    pub fn download_manifest(&self, manifest: &Manifest) {
        let mut out = Command::new("dotnet")
            .args(["downloader/DepotDownloader.dll", 
            "-app", &self.id.to_string(),
            "-filelist", "filter.txt",
            "-depot", &self.depot.to_string(),
            "-manifest", &manifest.id,
            "-os", "windows",
            "-dir", &String::from(self.dir.clone() + &manifest.name)
            ])
            .spawn()
            .expect("Failed");
        
        out.wait().unwrap();
    }

}