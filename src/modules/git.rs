use git2::{Repository, ResetType};

pub struct Git {
    pub dir: String
}

impl Git {
    pub fn new(dir: String) -> Self {
        return Self { dir }
    }

    pub fn initialize(&mut self) {
        let repo = match Repository::init(self.dir.clone()) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to initialize git repo: {}", e),
        };


    }
}