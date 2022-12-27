use std::fs::{ReadDir};
use std::{path::Path, fs, thread};
use std::time::Duration;

const BLACKHOLE_DIR_NAME: &str = "blackhole";

pub struct BlackHole {
    pub directory: Option<Directory>,
    interval_check: f32,
}

impl BlackHole {
    pub fn new() -> BlackHole {
        return BlackHole {
            directory: None,
            interval_check: 1.5
        };
    }

    pub fn set_interval(mut self, interval: f32) -> Self {
        self.interval_check = interval;
        return self;
    }

    pub fn from_directory(mut self, path: String) -> Self {
        self.directory = Some(Directory::new(path));
        return self;
    }

    pub fn start(&self) {
        self.build_hole();

        let duration: Duration = Duration::from_secs_f32(self.interval_check);
        let directory: &Directory = self.directory.as_ref().unwrap();
        loop {
            directory.delete_dir_content();
            thread::sleep(duration);
        }
    }

    fn build_hole(&self) {
        self.directory.as_ref().unwrap().create();
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

    fn get_full_path_str(&self) -> String {
        let pieces = vec![String::from(&self.path), String::from(std::path::MAIN_SEPARATOR), String::from(&self.name)];
        return pieces.join("");
    }

    fn exists(&self) -> bool {
        let full_path = self.get_full_path_str();
        let path = Path::new(&full_path);
        return path.exists();
    }

    pub fn create(&self) {
        if self.exists() {
            return;
        }

        let full_path: String = self.get_full_path_str();
        let path: &Path = Path::new(&full_path);

        match fs::create_dir_all(path) {
            Ok(_) => println!("Directory {} created", full_path),
            Err(e) => println!("Error creating directory: {}", e),
        }
    }

    fn delete_dir_content(&self) {
        let current_path_str = self.get_full_path_str();
        self.delete_dir_content_for_path(current_path_str);
    }

    fn delete_dir_content_for_path(&self, path_str: String) {
        let from_path: &Path = Path::new(&path_str);
        let result = self.read_dir(from_path);
        
        if result.is_none() {
            return;
        }

        let read_dir: ReadDir = result.unwrap();
        for entry in read_dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let path_str = String::from(path.to_str().unwrap());
            let metadata = entry.metadata().ok().unwrap();
            
            if metadata.is_dir() {
                self.delete_dir_content_for_path(path_str);
                match fs::remove_dir(path) {
                    Ok(()) => println!("Directory removed"),
                    Err(e) => println!("Error removing dir, e => {}", e)
                };
            }
            else {
                match fs::remove_file(path.as_path()) {
                    Ok(()) => continue,
                    Err(_) => continue,
                }
            }
        }
    }

    fn read_dir(&self, from_path: &Path) -> Option<ReadDir> {
        let path: &Path = Path::new(&from_path);

        match fs::read_dir(path) {
            Ok(entries) => return Some(entries),
            Err(e) => {
                println!("Error reading {} directory, error => {}", self.get_full_path_str(), e);
                return None;
            },
        };
    }

}

