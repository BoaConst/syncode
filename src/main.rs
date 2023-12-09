mod cli;
mod repository;
mod utils;
mod errors;

use std::path::Path;
use crate::cli::build_cli;

fn main() {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            let repo_path = sub_m.value_of("directory").unwrap_or(".");
            match repository::init(Path::new(repo_path)) {
                Ok(()) => println!("Repository initialized at {}", repo_path),
                Err(e) => eprintln!("Error initializing repository: {}", e),
            }
        }
        Some(("add", sub_m)) => {
            let repo_path = Path::new(".");
            let files: Vec<String> = sub_m.values_of("files").unwrap().map(String::from).collect();
            match repository::add(&repo_path, files) {
                Ok(()) => println!("Files added to tracking."),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(("remove", sub_m)) => {
            let repo_path = Path::new(".");
            let file = sub_m.value_of("file").unwrap().to_string();
            match repository::remove(&repo_path, file) {
                Ok(()) => println!("File removed from tracking."),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(("commit", sub_m)) => {
            let repo_path = Path::new(".");
            let message = sub_m.value_of("message").unwrap().to_string();
            match repository::commit(&repo_path, message) {
                Ok(()) => println!("Changes committed."),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        _ => {}
    }
}
