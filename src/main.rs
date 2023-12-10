#![allow(warnings, unused)]

pub mod machine_hiding;
pub mod user_hiding;
pub mod repository_hiding;
use clap::{Command, Arg};
use uuid;
use serde;
use serde_json;


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
        )
        .subcommand(
            Command::new("add")
                .about("Add a file to the staging area")
                .arg(Arg::new("file").help("The file to add").required(true))
        )
        // .subcommand(
        //     Command::new("remove")
        //         .about("Remove a file from the staging area")
        //         .arg(Arg::new("file").help("The file to remove").required(true))
        // )
        // .subcommand(
        //     Command::new("commit")
        //         .about("Commit the staged files")
        //         .arg(Arg::new("message").help("The commit message").required(false))
        // )
        .get_matches();

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

        Some(("add", add_matches)) => {
                let mut args = Vec::new();
                let mut path = String::new();
                if add_matches.args_present() {
                    path = add_matches.get_one::<String>("file").unwrap().to_string();;
                }
                args.push(&path);
                user_hiding::command_parser::command("add".to_string(), args);
            }


        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
