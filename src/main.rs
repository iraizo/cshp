use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command, ptr::read,
};

use modules::steamcmd::Steamcmd;

use crate::modules::{git::Git, types::{Manifest, Configuration}};

pub mod modules {
    pub mod git;
    pub mod steamcmd;
    pub mod types;
}

fn main() {
    let file = File::open("config.json").unwrap();
    
    let reader = BufReader::new(file);
    let config: Configuration = serde_json::from_reader(reader).unwrap();

    let cmd = Steamcmd::new(config.id, config.depot, config.download_out.clone() + "/");
    println!("[*] Fetching depot..");
    let mut depot = cmd.fetch_depot().clone();

    println!("[+] Done fetching depots.");
    for manifest in depot.manifests.clone() {
        let path = cmd.dir.clone() + &manifest.name;
        std::fs::create_dir_all(path).unwrap();

        println!(
            "[*] Downloading manifest {} ID: {}",
            &manifest.name, &manifest.id
        );
        cmd.download_manifest(&manifest);
    }

    println!("[+] Done downloading all manifests");
    println!("[*] Decompiling Assemblies..");

    let file = File::open(config.filter).unwrap();
    let filter: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    for manifest in &depot.manifests {
        for file in &filter {
            let mut out = Command::new("/Users/raizo/.dotnet/tools/ilspycmd")
                .args([
                    "-p",
                    "-o",
                    &String::from(config.pseudo_out.clone() + "/" + &manifest.name),
                    &String::from(config.download_out.clone() + "/" + &manifest.name + "/" + file),
                ])
                .spawn()
                .expect("Failed");

            out.wait().unwrap();
            println!("[*] Decompilation done of manifest {}", manifest.name);
        }
    }

    for manifest in &depot.manifests {
        let mut repo = Git::new(config.repo.clone(), config.pseudo_out.clone(), manifest.name.clone());
        repo.update();
        println!("[*] Updated branch of {:?}", manifest.name);
    }

    let mut repo = Git::new(config.repo.clone(),config.pseudo_out.clone(),config.main_manifest.clone());
    repo.push();

    let index = depot
        .manifests
        .iter()
        .position(|x| x.name == config.main_manifest.clone())
        .unwrap();
    depot.manifests.remove(index);

    for manifest in &depot.manifests {
        let mut repo = Git::new(config.repo.clone(), config.pseudo_out.clone(), manifest.name.clone());
        repo.push();
        println!("[*] Pushed branch of {:?}", manifest.name);
    }
}
