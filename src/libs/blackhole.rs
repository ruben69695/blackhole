use std::{path::Path, fs, thread};
use std::time::Duration;

const BLACKHOLE_DIR_NAME: &str = "blackhole";

pub struct BlackHole {
    pub directory: Directory,
    interval_check: f32,
}

impl BlackHole {
    pub fn new(interval: f32) -> BlackHole {
        return BlackHole {
            directory: Directory::new(String::new()),
            interval_check: interval
        };
    }

    pub fn from_directory(mut self, path: String) -> Self {
        self.directory = Directory::new(path);
        return self;
    }

    pub fn start(&self) {
        self.build_hole();

        let duration = Duration::from_secs_f32(self.interval_check);
        loop {

            thread::sleep(duration)
        }
    }

    fn build_hole(&self) {
        self.directory.create();
    }
}

pub struct Directory {
    pub path: String,
    pub name: String,
}

impl Directory {
    fn new(base_path: String) -> Directory {
        return Directory { 
            path: base_path, 
            name: String::from(BLACKHOLE_DIR_NAME),
        };
    }

    fn get_full_path(&self) -> String {
        let pieces = vec![String::from(&self.path), String::from(std::path::MAIN_SEPARATOR), String::from(&self.name)];
        return pieces.join("");
    }

    fn exists(&self) -> bool {
        let full_path = self.get_full_path();
        let path = Path::new(&full_path);
        return path.exists();
    }

    pub fn create(&self) {
        if self.exists() {
            return;
        }

        let full_path = self.get_full_path();
        let path = Path::new(&full_path);

        match fs::create_dir_all(path) {
            Ok(_) => println!("Directory {} created", full_path),
            Err(e) => println!("Error creating directory: {}", e),
        }
    }
}

