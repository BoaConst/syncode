/*
potential solution for command parsing and syntax error handling
require "cargo add clap" to compile
*/

mod repo;

use clap::{Command, Arg, arg};

fn main() {
    let matches = Command::new("arc")
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
            Command::new("print")
                .about("Prints a repository")
                .arg(Arg::new("path").help("The repository user wants to print").required(false)),
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
                .about("Add specific files that user wants to track")
                .arg(Arg::new("path").help("Path of the added file").required(true))
        )
        .subcommand(
            Command::new("remove")
                .about("Remove specific files from tracking list")
                .arg(Arg::new("path").help("Path of the removed file").required(true))
        )
        .subcommand(
            Command::new("heads")
                .about("Show the current heads")
        )
        .subcommand(
            Command::new("diff")
                .about("Check the changes between revisions")
        )
        .subcommand(
            Command::new("cat")
                .about("Inspect a file of a given revision")
                .arg(Arg::new("path").help("File path of inspected file").required(true))
        )
        .subcommand(
            Command::new("checkout")
                .about("Check out a specific revision")
                .arg(Arg::new("rev").help("Revision to checkout to").required(true))
        )
        .subcommand(
            Command::new("commit")
                .about("Commit changes")
        )
        .subcommand(
            Command::new("merge")
                .about("Merge two revisions")
                .arg(Arg::new("rev1").help("First revision to merge").required(true))
                .arg(Arg::new("rev2").help("Second revision to merge").required(true))
        )
        .subcommand(
            Command::new("push")
                .about("Push changes")
        )
        .subcommand(
            Command::new("pull")
                .about("Pull changes")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            todo!()
            /*
            check root conflict handling

            */
        }
        Some(("print", print_matches)) => {
            todo!()
        }
        Some(("clone", clone_matches)) => {
            todo!()
        }
        Some(("add", add_matches)) => {
            todo!()
        }
        Some(("remove", remove_matches)) => {
            todo!()
        }
        Some(("heads", heads_matches)) => {
            todo!()
        }
        Some(("diff", diff_matches)) => {
            todo!()
        }
        Some(("cat", cat_matches)) => {
            todo!()
        }
        Some(("checkout", checkout_matches)) => {
            todo!()
        }
        Some(("commit", commit_matches)) => {
            todo!()
        }
        Some(("merge", merge_matches)) => {
            todo!()
        }
        Some(("push", push_matches)) => {
            todo!()
        }
        Some(("pull", pull_matches)) => {
            todo!()
        }
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}

