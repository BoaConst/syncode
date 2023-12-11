/*
# Module B.3 : User Interface Module

**Designer:** Helia 
*/

use super::machine_hiding;
use super::repository_hiding::initialization::*;
use super::repository_hiding::inspection::*;
use super::repository_hiding::staging::*;
use super::repository_hiding::synchronization::*;
use super::repository_hiding;
use crate::user_hiding;
use std::io;
use std::io::Write;

pub fn execute_command(cmd_name: String, args: Vec<&String>) -> Result<(), DvcsError> {
    let cwd = machine_hiding::file_system_operations::get_cwd();
    let cmd_result = user_hiding::command_parser::parse_command(cmd_name.clone());
    // Handle the Result returned by parse_command
    let cmd = match cmd_result {
        Ok(command) => command,
        Err(err) => {
            eprintln!("Error: {}", err.to_string());
            return Err(err);
        }
    };

    // validate command using command_parser::validate_command
    match user_hiding::command_parser::validate_command(&cmd, args.clone()) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error: {}", err.to_string());
            return Err(err);
        }
    }

    match cmd {
        DvcsCommand::Init => {
            let repo_root_path = &args[0];
            match init(repo_root_path) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Error: {}", err.to_string());
                    return Err(err);
                }
            }
        },
        DvcsCommand::Clone => {
            let src = &args[0];
            let dst = &args[1];
            match clone(src, dst) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Error: {}", err.to_string());
                    return Err(err);
                }
            }
        },

        // TODO: Add and/or edit the rest of the commands as needed
        DvcsCommand::Add => {
            let file_abs_path = machine_hiding::file_system_operations::join_paths(&cwd, &args[0]);
            // println!("abs path at {}", file_abs_path);

            let repo_root_path = machine_hiding::file_system_operations::find_repo_root_path(&cwd);
            // println!("repo find at {}", repo_root_path);
            let file_rel_path = machine_hiding::file_system_operations::find_rel_path(&repo_root_path, &file_abs_path);
            // println!("file rel path find at {}", file_abs_path);
            let mut r = repository_hiding::initialization::open(&repo_root_path);

            // match r.add_file(&file_rel_path) {
            match r.add_file(&file_abs_path) {
                Ok(()) => {r.save();}
                Err(err) => {
                    eprintln!("Error: {}", err.to_string());
                    return Err(err);
                }
            }
            // r.add_file(&file_rel_path);
            // r.save();

            println!("{}", r);
        },
        DvcsCommand::Commit => {
            let repo_root_path = machine_hiding::file_system_operations::find_repo_root_path(&cwd);
            let mut repo = repository_hiding::initialization::open(&repo_root_path);
            repo.commit();
            repo.save();
        },
        // DvcsCommand::Remove => {
        //     let file = &args[0];
        //     match remove(file) {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //     }
        // },
        // DvcsCommand::Log => {
        //     match log() {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //     }
        // },
        DvcsCommand::Checkout => {
            let repo_root_path = machine_hiding::file_system_operations::find_repo_root_path(&cwd);
            let mut repo = repository_hiding::initialization::open(&repo_root_path);
            repo.checkout(&args[0]);
            repo.save();

        },
        DvcsCommand::Heads => {
            let repo_root_path = machine_hiding::file_system_operations::find_repo_root_path(&cwd);
            let mut repo = repository_hiding::initialization::open(&repo_root_path);
            println!("current heads are at: {}", repo.get_head_rev_str());
        },
        // DvcsCommand::Status => {
        //     match status() {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //     }
        // },
        // DvcsCommand::Push => {
        //     match push() {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //     }
        // },
        // DvcsCommand::Pull => {
        //     match pull() {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //     }
        // },
        // DvcsCommand::Merge => {
        //     let commit1 = &args[0];
        //     let commit2 = &args[1];
        //     match merge(commit1, commit2) {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //     }
        // },
        // DvcsCommand::Heads => {
        //     match heads() {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //     }
        //
        // },
        // DvcsCommand::Cat => {
        //     let file = &args[0];
        //     match cat(file) {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //
        //     }
        // },
        // DvcsCommand::Diff => {
        //     let commit1 = &args[0];
        //     let commit2 = &args[1];
        //     match diff(commit1, commit2) {
        //         Ok(()) => {}
        //         Err(err) => {
        //             eprintln!("Error: {}", err.to_string());
        //             return Err(err);
        //         }
        //
        //     }
        // },
        
        _ => {
            return Err(DvcsError::InvalidCommand);
        },
    }
    Ok(())
}


// /// This function is used to print the DVCS prompt.
// pub fn prompt_user() {
//     print!("dvcs> ");
//     io::stdout().flush().unwrap();
// }

// /// This function is used to print the DVCS help message.
// fn print_help() {
//     println!("init - Create an empty DVCS repository in the current directory");
//     println!("add <file> - Add a file to the DVCS repository");
//     println!("commit <message> - Record changes to the repository");
//     println!("remove <file> - Remove a file from the DVCS repository");
//     println!("log - Show commit logs");
//     println!("checkout <commit> - Restore working directory to a previous commit");
//     println!("status - Show the status of files in the current directory");
//     println!("push - Push changes from local repository to remote repository");
//     println!("pull - Pull changes from remote repository to local repository");
//     println!("clone <url> - Clone a remote repository into a new directory");
//     println!("help - Show the help message");
//     println!("exit - Exit the DVCS");
// }

// // use crate::repository_hiding::initialization::*;

// /// This function is used to execute the DVCS commands.
// fn execute_command(command: &DvcsCommand, args: &Vec<String>) -> Result<(), DvcsError> {
//     match command {
//         DvcsCommand::Init => {
//             // call init_repository
//             init();
//         }
//         DvcsCommand::Add => {
//             // call add_file
//             add(&args[0]);
//         }
//         DvcsCommand::Commit => {
//             // call commit_changes
//             commit(&args[0]);
//         }
//         DvcsCommand::Remove => {
//             // call remove_file
//             remove(&args[0]);
//         }
//         DvcsCommand::Log => {
//             // call show_commit_logs
//             log();
//         }
//         DvcsCommand::Checkout => {
//             // call checkout_commit
//             checkout(&args[0]);
//         }
//         DvcsCommand::Status => {
//             // call show_status
//             status();
//         }
//         DvcsCommand::Help => {
//             print_help();
//         }
//         DvcsCommand::Exit => {
//             println!("Exiting DVCS...");
//             std::process::exit(0);
//         }
//         DvcsCommand::Push => {
//             // call push_changes
//             push();
//         }
//         DvcsCommand::Pull => {
//             // call pull_changes
//             pull();
//         }
//         DvcsCommand::Clone => {
//             // call clone_repository
//             clone(&args[0]);
//         }

//         // TO DO: add more commands as needed
//     }
//     Ok(())
// }

// /// This function is used to show the interface for the DVCS.
// pub fn show_interface() {
//     loop {
//         prompt_user();
//         let mut user_input = String::new();
//         io::stdin().read_line(&mut user_input).unwrap();
//         let user_input = user_input.trim();
//         let mut args: Vec<String> = user_input.split_whitespace().map(|s| s.to_string()).collect();
//         let command = parse_command(&args[0]);
//         match command {
//             Ok(command) => {
//                 args.remove(0);
//                 let result = validate_command(&command, &args);
//                 match result {
//                     Ok(_) => {
//                         let result = execute_command(&command, &args);
//                         match result {
//                             Ok(_) => {}
//                             Err(e) => {
//                                 handle_error(&e);
//                             }
//                         }
//                     }
//                     Err(e) => {
//                         handle_error(&e);
//                     }
//                 }
//             }
//             Err(e) => {
//                 handle_error(&e);
//             }
//         }
//     }
// }

// /// This function is used to handle the errors using the Display trait implemented in user_hiding::syntax_checker_error_handling.
// fn handle_error(error: &DvcsError) {
//     println!("{}", error);
// }




// #[cfg(test)]
// mod tests {
//     use super::*;

//     // Test ID: 1
//     #[test]
//     fn test_execute_command_success() {
//         // TO DO: add test cases

//         let command = DvcsCommand::Init;
//         let args = vec![];
//         let result = execute_command(&command, &args);
//         assert_eq!(result, Ok(()));
//     }

//     // Test ID: 2
//     #[test]
//     fn test_execute_command_failure() {
//         // TO DO: add test cases

//         let command = DvcsCommand::Add;
//         let args = vec![];
//         let result = execute_command(&command, &args);
//         assert_eq!(result, Err(DvcsError::InvalidNumberOfArguments));

//         let command = DvcsCommand::Commit;
//         let args = vec![String::from("invalidArgs")];
//         let result = execute_command(&command, &args);
//         assert_eq!(result, Err(DvcsError::InvalidArguments));

//     }

//     // Test ID: 3
//     #[test]
//     fn test_show_interface() {
//         // TO DO: add test cases

//         let result = show_interface();
//         assert_eq!(result, Ok(()));
//     }

//     // Test ID: 4
//     #[test]
//     fn test_handle_error() {
//         // TO DO: add test cases

//         let error = DvcsError::InvalidNumberOfArguments;
//         let result = handle_error(&error);
//         assert_eq!(result, Ok(()));
//     }
// }
