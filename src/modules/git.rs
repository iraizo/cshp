use std::{
    fs::{self, copy, read_dir},
    io,
    path::Path,
    process::{Command, Stdio},
};

use fs_extra::dir::CopyOptions;
#[derive(Clone)]
pub struct Git {
    pub dir: String,
    pub pseudo_dir: String,
    pub manifest_name: String,
}

impl Git {
    pub fn new(dir: String, pseudo_dir: String, manifest_name: String) -> Self {
        return Self { dir, manifest_name, pseudo_dir };
    }

    pub fn update(&mut self) {
        self.add();
        self.commit();
    }

    pub fn push(&mut self) {
        let mut out = Command::new("git")
           // .current_dir(self.dir.clone())
            .args(["--git-dir=.git", "push", "origin", &self.manifest_name])
            .stdin(Stdio::null())
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }

    pub fn add(&mut self) {
        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
            .args(["--git-dir=.git", "checkout", "-f", &self.manifest_name])
            .stdin(Stdio::null())
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
        //    .current_dir(self.dir.clone())
            .stdin(Stdio::null())
            .args(["--git-dir=.git", "add", "."])
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }

    pub fn commit(&mut self) {
        let mut out = Command::new("git")
          //  .current_dir(self.dir.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(["--git-dir=.git", "commit", "-m", &("Updated ".to_owned() + &self.manifest_name)])
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }
}
