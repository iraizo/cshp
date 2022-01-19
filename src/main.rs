use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
};

use modules::steamcmd::Steamcmd;

use crate::modules::{git::Git, types::Manifest};

pub mod modules {
    pub mod git;
    pub mod steamcmd;
    pub mod types;
}

fn main() {
    let cmd = Steamcmd::new(258550, 258551, "out/".to_string());
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

    let file = File::open("filter.txt").unwrap();
    let filter: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    for manifest in &depot.manifests {
        std::fs::create_dir_all(&String::from("pseudo/".to_owned() + &manifest.name)).unwrap();
        for file in &filter {
            let mut out = Command::new("/Users/raizo/.dotnet/tools/ilspycmd")
                .args([
                    "-p",
                    "-o",
                    &String::from("pseudo/".to_owned() + &manifest.name),
                    &String::from("out/".to_owned() + &manifest.name + "/" + file),
                ])
                .spawn()
                .expect("Failed");

            out.wait().unwrap();
            println!("[*] Decompilation done of manifest {}", manifest.name);
        }
    }

    for manifest in &depot.manifests {
        let mut repo = Git::new("repo".to_string(), manifest.name.clone());
        repo.update();
        println!("[*] Updated branch of {:?}", manifest.name);
    }

    /* TODO: dont hardcode this */
    let main_repo = "public";

    let mut repo = Git::new("repo".to_string(), main_repo.to_string());
    repo.push();

    let index = depot
        .manifests
        .iter()
        .position(|x| x.name == main_repo)
        .unwrap();
    depot.manifests.remove(index);

    for manifest in &depot.manifests {
        let mut repo = Git::new("repo".to_string(), manifest.name.clone());
        repo.push();
        println!("[*] Pushed branch of {:?}", manifest.name);
    }
}
