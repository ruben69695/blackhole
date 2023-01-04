use std::path::Path;
use std::fs;
use std::thread;
use std::time::Duration;

use super::directory::Directory;
use super::utils;

const BLACKHOLE_DIR_NAME: &str = "blackhole";

pub struct BlackHole {
    pub directory: Option<Directory>,
    interval_check: f32,
    files: Vec<String>,
}

impl BlackHole {
    pub fn new() -> BlackHole {
        return BlackHole {
            directory: None,
            interval_check: 1.5,
            files: Vec::new()
        };
    }

    pub fn set_interval(mut self, interval: f32) -> Self {
        self.interval_check = interval;
        return self;
    }

    pub fn from_directory(mut self, base_path: String) -> Self {
        let dir_name: String = BLACKHOLE_DIR_NAME.to_string();
        self.directory = Some(Directory::new(base_path, dir_name));
        return self;
    }

    pub fn start_hole(&self) {
        self.build_hole();

        let duration: Duration = Duration::from_secs_f32(self.interval_check);
        let directory: &Directory = self.directory.as_ref().unwrap();
        loop {
            directory.delete_dir_content();
            thread::sleep(duration);
        }
    }

    pub fn set_files(mut self, file_paths: Vec<String>) -> Self {
        self.files = file_paths;
        return self;
    }

    pub fn eat_files(&self) {
        if self.files.is_empty() {
            return;
        }

        let paths: Vec<&Path> = utils::filesystem::convert_to_paths(&self.files);

        for p in paths {
            
            if !p.exists() {
                continue;
            }

            if p.is_dir() {
                match fs::remove_dir_all(p) {
                    Ok(_) => continue,
                    Err(_) => println!("> Error deleting directory '{}'", p.to_str().unwrap().to_string()),
                }
            }

            match fs::remove_file(p) {
                Ok(_) => continue,
                Err(_) => println!("> Error deleting file '{}'", p.to_str().unwrap().to_string()),
            }
        }
    }

    fn build_hole(&self) {
        self.directory.as_ref().unwrap().create();
    }
}