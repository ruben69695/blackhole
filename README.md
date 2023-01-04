![GitHub](https://img.shields.io/github/license/ruben69695/blackhole?color=purple)
![GitHub last commit](https://img.shields.io/github/last-commit/ruben69695/blackhole)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/ruben69695/blackhole?color=purple)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ruben69695/blackhole?color=purple)

# Blackhole
A black hole made with 🦀 Rust, proceed with caution it makes disappear everything goes inside or you pass to it.

## 📦 Language and dependencies
- Rust 1.65.0
- Cargo 1.65.0
- exitcode 1.1.2

## ✏️ Getting started

- Parameters
    ```zsh
    --help, -h      print this message
    --version, -v   print current version
    --eat, -e       delete files or directories, if multiple items separate them by spaces
    <base_path>     absolute path where to create the blackhole
    <interval>      [optional] indicates the time in seconds it takes for the black hole to absorb data
    ```
- Usage
    ```zsh
    blackhole --help
    blackhole --version
    blackhole <base_path> <interval>
    blackhole --eat <file_path_one> <file_path_two> <directory_path>
    ```
    
- Examples
    ```zsh
    blackhole /home/user/Dowloads
    blackhole /home/user/Downloads 1.5
    blackhole --eat /file.tkt /cat.c /home/ra/directory
    ```

## 🏭 Building release
1. Adding platform targets if not added: MacOS, Linux, Windows
    ```zsh
    rustup target add aarch64-apple-darwin
    rustup target add x86_64-apple-darwin
    rustup target add x86_64-unknown-linux-gnu
    rustup target add x86_64-pc-windows-gnu
    ```

2. Build release for every platform
- Multiple way
    ```zsh
    cargo build --release --target=aarch64-apple-darwin --target=x86_64-apple-darwin --target=x86_64-unknown-linux-gnu --target=x86_64-pc-windows-gnu
    ```

- Individual way
    ```zsh
    cargo build --target=aarch64-apple-darwin --release
    cargo build --target=x86_64-apple-darwin --release
    cargo build --target=x86_64-unknown-linux-gnu --release
    cargo build --target=x86_64-pc-windows-gnu --release
    ```


## 💻 Development environment
- Fedora 37 and MacOS Monterey
- Codium 1.74

## 🚀 Runs on
- Linux
- MacOS (Apple Silicon and Intel)
- Windows
