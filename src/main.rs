mod libs;

use crate::libs::blackhole::BlackHole;

fn main() {
    println!("This is a black hole, proceed with caution!");
    let bh = BlackHole::new(1.5).from_directory(String::from("/home/rubenarrebola/Downloads"));
    bh.start();
}