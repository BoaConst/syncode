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
                .arg(Arg::new("repo").help("repository name").required(false))
            // false with cwd default in implementation
        )
        .subcommand(
            Command::new("clone")
                .about("Clone a repository")
                .arg(Arg::new("src").help("The source repository").required(true))
                .arg(Arg::new("dst").help("The destination repository").required(false))
        )
        .subcommand(
            Command::new("cat")
                .about("Display the contents of a file")
                .arg(Arg::new("file").help("The file to display").required(true))
        )
        .subcommand(
            Command::new("add")
                .about("Add a file to the staging area")
                .arg(Arg::new("file").help("The file to add").required(true))
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a file from the staging area")
                .arg(Arg::new("file").help("The file to remove").required(true))
        )
        .subcommand(
            Command::new("commit")
                .about("Commit the staged files")
                .arg(Arg::new("message").help("The commit message").required(false))
        )
        .subcommand(
            Command::new("log")
                .about("Display the commit history")
        )
        .subcommand(
            Command::new("checkout")
                .about("Checkout a commit")
                .arg(Arg::new("commit").help("The commit to checkout").required(true))
        )
        .subcommand(
            Command::new("status")
                .about("Display the status of the repository")
        )
        .subcommand(
            Command::new("heads")
                .about("Display the heads of the repository")
        )
        .subcommand(
            Command::new("diff")
                .about("Display the differences between two commits")
                .arg(Arg::new("commit1").help("The first commit").required(true))
                .arg(Arg::new("commit2").help("The second commit").required(true))
        )
        .subcommand(
            Command::new("merge")
                .about("Merge two revisions")
                .arg(Arg::new("repo_path").help("The repository path").required(true))
                .arg(Arg::new("revision1").help("The first revision").required(true))
                .arg(Arg::new("revision2").help("The second revision").required(true))
        )
        .subcommand(
            Command::new("push")
                .about("Push a repository to a remote")
                .arg(Arg::new("repo_path").help("The repository path").required(true))
                .arg(Arg::new("remote").help("The remote to push to").required(true))
        )
        .subcommand(
            Command::new("pull")
                .about("Pull a repository from a remote")
                .arg(Arg::new("repo_path").help("The repository path").required(true))
                .arg(Arg::new("remote").help("The remote to pull from").required(true))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let mut args = Vec::new();
            let mut directory = String::new();
            if init_matches.args_present() {
                let name = init_matches.get_one::<String>("repo").unwrap().to_string();
                let cwd = machine_hiding::file_system_operations::get_cwd();
                directory = cwd + "/" + &name;
            } else {
                directory = machine_hiding::file_system_operations::get_cwd();
            }
            args.push(&directory);
            user_hiding::user_interface::execute_command("init".to_string(), args);
        }
        Some(("clone", clone_matches)) => {
            let mut args = Vec::new();
            let src = clone_matches.get_one::<String>("src").unwrap().to_string();
            // check if dst is present as one of the provided args, if not, use cwd
            let dst = clone_matches.get_one::<String>("dst").map_or_else(
                || machine_hiding::file_system_operations::get_cwd(),
                |d| d.to_string(),
            );

            args.push(&src);
            args.push(&dst);
            user_hiding::user_interface::execute_command("clone".to_string(), args);
        }
        Some(("merge", merge_matches)) => {
            let mut args = Vec::new();
            let commit1 = merge_matches.get_one::<String>("revision1").unwrap().to_string();
            let commit2 = merge_matches.get_one::<String>("revision2").unwrap().to_string();
            let repo_path = merge_matches.get_one::<String>("repo_path").unwrap().to_string();
            args.push(&commit1);
            args.push(&commit2);
            args.push(&repo_path);
            user_hiding::user_interface::execute_command("merge".to_string(), args);
        }

        // TODO: add and/or edit the rest of the commands as needed
        Some(("cat", cat_matches)) => {
            let mut args = Vec::new();
            let file = cat_matches.get_one::<String>("file").unwrap().to_string();
            args.push(&file);
            user_hiding::user_interface::execute_command("cat".to_string(), args);
        }
        Some(("add", add_matches)) => {
            let mut args = Vec::new();
            let mut path = String::new();
            if add_matches.args_present() {
                path = add_matches.get_one::<String>("file").unwrap().to_string();;
            }
            args.push(&path);
            user_hiding::user_interface::execute_command("add".to_string(), args);
        }
        Some(("remove", remove_matches)) => {
            let mut args = Vec::new();
            let file = remove_matches.get_one::<String>("file").unwrap().to_string();
            args.push(&file);
            user_hiding::user_interface::execute_command("remove".to_string(), args);
        }
        Some(("commit", commit_matches)) => {
            let args = Vec::new();
            user_hiding::user_interface::execute_command("commit".to_string(), args);
        }
        Some(("log", _)) => {
            let mut args = Vec::new();
            user_hiding::user_interface::execute_command("log".to_string(), args);
        }
        Some(("checkout", checkout_matches)) => {
            let mut args = Vec::new();
                let mut rev = String::new();
                if checkout_matches.args_present() {
                    rev = checkout_matches.get_one::<String>("commit").unwrap().to_string();
                }
                args.push(&rev);
                user_hiding::user_interface::execute_command("checkout".to_string(), args);
        }
        Some(("status", _)) => {
            let mut args = Vec::new();
            user_hiding::user_interface::execute_command("status".to_string(), args);
        }
        Some(("heads", _)) => {
            let mut args = Vec::new();
            user_hiding::user_interface::execute_command("heads".to_string(), args);
        }
        Some(("diff", diff_matches)) => {
            let mut args = Vec::new();
            let commit1 = diff_matches.get_one::<String>("commit1").unwrap().to_string();
            let commit2 = diff_matches.get_one::<String>("commit2").unwrap().to_string();
            args.push(&commit1);
            args.push(&commit2);
            user_hiding::user_interface::execute_command("diff".to_string(), args);
        }
        Some(("push", push_matches)) => {
            let mut args = Vec::new();
            let remote = push_matches.get_one::<String>("remote").unwrap().to_string();
            let repo_path = push_matches.get_one::<String>("repo_path").unwrap().to_string();
            args.push(&remote);
            args.push(&repo_path);
            user_hiding::user_interface::execute_command("push".to_string(), args);
        }
        Some(("pull", pull_matches)) => {
            let mut args = Vec::new();
            let remote = pull_matches.get_one::<String>("remote").unwrap().to_string();
            let repo_path = pull_matches.get_one::<String>("repo_path").unwrap().to_string();
            args.push(&remote);
            args.push(&repo_path);
            user_hiding::user_interface::execute_command("pull".to_string(), args);
        }

        None => println!("Please enter a command! Type 'cargo run help' for more information."),
        _ => unreachable!(),
    }
}
