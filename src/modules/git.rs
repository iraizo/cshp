use std::{
    fs::{self, copy, read_dir},
    io,
    path::Path,
    process::{Command, Stdio},
};
#[derive(Clone)]
pub struct Git {
    pub dir: String,
    pub manifest_name: String,
}

impl Git {
    pub fn new(dir: String, manifest_name: String) -> Self {
        return Self { dir, manifest_name };
    }

    pub fn update(&mut self) {
        self.add();
        self.commit();
    }

    pub fn push(&mut self) {
        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
            .args(["push", "origin", &self.manifest_name])
            .stdin(Stdio::null())
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }

    pub fn add(&mut self) {
        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
            .args(["checkout", "-f", &self.manifest_name])
            .stdin(Stdio::null())
            .spawn()
            .expect("Failed");

        out.wait().unwrap();

        copy(
            &String::from("pseudo/".to_owned() + &self.to_owned().manifest_name),
            &self.dir.clone(),
        )
        .unwrap();

        /*    let dir = read_dir(&String::from(
            "pseudo/".to_owned() + &self.to_owned().manifest_name,
        ))
        .unwrap();
        for file in dir {
            copy(
                file.as_ref().unwrap().path(),
                String::from(self.dir.clone() + "/" + file.unwrap().file_name().to_str().unwrap()),
            )
            .unwrap();
        }*/

        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
            .stdin(Stdio::null())
            .args(["add", "."])
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }

    pub fn commit(&mut self) {
        let mut out = Command::new("git")
            .current_dir(self.dir.clone())
            .stdin(Stdio::null())
            .args(["commit", "-m", "Update"])
            .spawn()
            .expect("Failed");

        out.wait().unwrap();
    }
}
