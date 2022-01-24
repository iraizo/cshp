use std::process::{Command, Stdio};

use fs_extra::dir::CopyOptions;
#[derive(Clone)]
pub struct Git {
    pub dir: String,
    pub pseudo_dir: String,
    pub manifest_name: String,
    pub main_manifest_name: String
}

impl Git {
    pub fn new(dir: String, pseudo_dir: String, manifest_name: String, main_manifest_name: String) -> Self {
        return Self { dir, manifest_name, pseudo_dir, main_manifest_name };
    }

    pub fn update(&mut self) {
        self.rebase();
        self.add();
        self.commit();
    }

    pub fn push(&mut self) {
        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
            .args(["push", "-f", "origin", &self.manifest_name, "-v"])
            .stdin(Stdio::null())
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }

    pub fn rebase(&mut self) {
        let mut out = Command::new("git")
        .current_dir(self.dir.clone())
        .args(["rebase", &self.main_manifest_name])
    //    .stdout(Stdio::null())
        .spawn()
        .expect("Failed");

        out.wait().unwrap();
    }

    pub fn add(&mut self) {
        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
            .args(["checkout", &self.manifest_name])
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed");

        out.wait().unwrap();

        let mut opt = CopyOptions::new();
        opt.overwrite = true;
        opt.content_only = true;

        fs_extra::dir::copy(
            &String::from(self.pseudo_dir.clone() + "/" + &self.to_owned().manifest_name),
            &self.dir.clone(), &opt
        )
        .unwrap();

        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
         //   .stdin(Stdio::null())
            .args(["add", "."])
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }

    pub fn commit(&mut self) {
        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
           // .stdout(Stdio::piped())
           // .stderr(Stdio::piped())
            .args(["commit", "-m", &("Updated ".to_owned() + &self.manifest_name)])
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }
}
