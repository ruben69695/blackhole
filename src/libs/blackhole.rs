use std::thread;
use std::time::Duration;

use super::directory::Directory;

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
        self.directory = Some(Directory::new(path, BLACKHOLE_DIR_NAME));
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