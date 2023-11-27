#![allow(warnings, unused)]

pub mod machine_hiding;
pub mod user_hiding;
pub mod repository_hiding;
use clap::{Command, Arg};


fn main() {
    let matches = Command::new("Prototype")
        .version("1.0")
        .about("DVCS")
        .author("SynCode")
        .subcommand(
            Command::new("init")
                .about("Initialize repository")
                .arg(Arg::new("directory").help("path directory").required(false))
            // false with cwd default in implementation
        )
        .subcommand(
            Command::new("clone")
                .about("Clone a repository")
                .arg(Arg::new("src").help("The source repository").required(true))
                .arg(Arg::new("dst").help("The destination repository").required(false))
        ).get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let mut args = Vec::new();
            let mut directory = String::new();
            if init_matches.args_present() {
                directory = init_matches.get_one::<String>("directory").unwrap().to_string();
            } else {
                directory = machine_hiding::file_system_operations::get_cwd();
            }
            args.push(&directory);
            user_hiding::command_parser::command("init".to_string(), args)
        }
        Some(("clone", _clone_matches)) => {
            todo!()
        }
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
