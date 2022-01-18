use std::{process::Command, fs::File, io::{BufReader, BufRead}};

use modules::steamcmd::Steamcmd;

pub mod modules {
    pub mod steamcmd;
    pub mod types;
    mod git;
}

fn main() {
    let cmd = Steamcmd::new(258550,258551, "out/".to_string());
    println!("[*] Fetching depot..");
    let depot = cmd.fetch_depot().clone();

    println!("[+] Done fetching depots.");
    for manifest in depot.manifests.clone() {
        let path = cmd.dir.clone() + &manifest.name;
        std::fs::create_dir_all(path).unwrap();

        println!("[*] Downloading manifest {} ID: {}", &manifest.name, &manifest.id);
        cmd.download_manifest(&manifest);
    }

    println!("[+] Done downloading all manifests");
    println!("[*] Decompiling Assemblies..");

    let file = File::open("filter.txt").unwrap();
    let filter: Vec<String> = BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();

    for manifest in &depot.manifests {
        std::fs::create_dir_all(&String::from("pseudo/".to_owned() + &manifest.name)).unwrap();
        for file in &filter {
            let mut out = Command::new("/Users/raizo/.dotnet/tools/ilspycmd")
            .args(["-o",
            &String::from("pseudo/".to_owned() + &manifest.name),
            &String::from("out/".to_owned() + &manifest.name + "/" + file)
            ])
            .spawn()
            .expect("Failed");

            out.wait().unwrap();
            println!("[*] Decompilation done of manifest {}", manifest.name);
        }
    }
}
