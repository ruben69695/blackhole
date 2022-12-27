mod libs;

use crate::libs::blackhole::BlackHole;
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

    let base_path: String = first_arg.to_string();

    if args_length > 2 {
        interval = match args[2].trim().parse() {
            Ok(num) => num,
            Err(_) => std::process::exit(exitcode::DATAERR),
        };
    }

    println!("This is a black hole, proceed with caution!");

    let bh: BlackHole = BlackHole::new()
        .from_directory(base_path)
        .set_interval(interval);

    bh.start();
}

fn exit(code: ExitCode, show_help: bool) {
    if show_help {
        print_help();
    }

    std::process::exit(code);
}

fn print_help() {
    println!("Blackhole CLI program

    USAGE:
        blackhole --help
        blackhole --version
        blackhole <base_path> <interval>
        
    EXAMPLE:
        blackhole /home/user/Dowloads
        blackhole /home/user/Downloads 1.5
        
    OPTIONS:
        --help, -h      print this message
        --version, -v   print current version
        <base_path>     the path where to create the blackhole
        <interval>      [optional] indicates the time in seconds it takes for the black hole to absorb"
    );
}