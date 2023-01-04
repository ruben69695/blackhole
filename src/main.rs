mod libs;

use crate::libs::blackhole::BlackHole;
use crate::libs::utils;
use exitcode::{self, ExitCode};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_length: usize = args.len();
    let mut interval: f32 = 1.25;

    if args_length < 2 {
        exit(exitcode::USAGE, true);
    }

    let first_arg: &str = args[1].trim();

    if first_arg == "--help" || first_arg == "-h" {
        exit(exitcode::OK, true);
    }

    if first_arg == "--version" || first_arg == "-v" {
        println!("blackhole {}", env!("CARGO_PKG_VERSION"));
        exit(exitcode::OK, false);
    }

    if first_arg == "--eat" || first_arg == "-e" {
        let file_paths = clone_args(&args[2..]);
        BlackHole::new()
            .set_files(file_paths)
            .eat_files();
        exit(exitcode::OK, false);
    }

    let base_path: String = first_arg.to_string();

    if args_length > 2 {
        interval = match args[2].trim().parse() {
            Ok(num) => num,
            Err(_) => std::process::exit(exitcode::DATAERR),
        };
    }

    println!("> This is a black hole, proceed with caution!");

    let bh: BlackHole = BlackHole::new()
        .from_directory(base_path)
        .set_interval(interval);

    bh.start_hole();
}

fn exit(code: ExitCode, show_help: bool) {
    if show_help {
        utils::cli::print_help();
    }

    std::process::exit(code);
}

fn clone_args(args: &[String]) -> Vec<String> {
    let mut file_paths: Vec<String> = Vec::new();
    for param in args {
        file_paths.push(param.clone());
    }
    return file_paths;
}