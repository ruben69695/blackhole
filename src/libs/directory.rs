use std::fs::ReadDir;
use std::{path::Path, fs};

pub struct Directory {
    pub path: String,
    pub name: String,
}

impl Directory {
    
    pub fn new(base_path: String, dir_name: &str) -> Directory {
        return Directory { 
            path: base_path, 
            name: String::from(dir_name),
        };
    }

    pub fn create(&self) {
        if self.exists() {
            return;
        }

        let full_path: String = self.get_full_path_str();
        let path: &Path = Path::new(&full_path);

        match fs::create_dir_all(path) {
            Ok(_) => println!("Directory created on {}", full_path),
            Err(e) => println!("Error creating directory on {}, with the next error {}", full_path, e),
        }
    }

    pub fn delete_dir_content(&self) {
        let current_path_str = self.get_full_path_str();
        self.delete_dir_content_for_path(current_path_str);
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