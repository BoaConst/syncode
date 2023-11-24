
mod user_interface {

    impl UserInterfaceModule {

        // Importing modules
        use std::io;
        use std::io::Write;
        use crate::syncode::user_hiding::*;
        use crate::syncode::repository_hiding::*;      // including DvcsCommand, DvcsError
        // use crate::dvcs::dvcs_auth::*;

        /// This function is used to print the DVCS prompt.
        pub fn prompt_user() {
            print!("dvcs> ");
            io::stdout().flush().unwrap();
        }

        /// This function is used to print the DVCS help message.
        fn print_help() {
            println!("init - Create an empty DVCS repository or reinitialize an existing one");
            println!("add <file> - Add a file to the DVCS repository");
            println!("commit <message> - Record changes to the repository");
            println!("remove <file> - Remove a file from the DVCS repository");
            println!("log - Show commit logs");
            println!("checkout <commit> - Restore working directory to a previous commit");
            println!("status - Show the status of files in the current directory");
            println!("push - Push changes from local repository to remote repository");
            println!("pull - Pull changes from remote repository to local repository");
            println!("clone <url> - Clone a remote repository into a new directory");
            println!("help - Show the help message");
            println!("exit - Exit the DVCS");
        }

        /// This function is used to execute the DVCS commands.
        fn execute_command(command: &DvcsCommand, args: &Vec<String>) -> Result<(), DvcsError> {
            match command {
                DvcsCommand::Init => {
                    // call init_repository
                    init();
                }
                DvcsCommand::Add => {
                    // call add_file
                    add(&args[0]);
                }
                DvcsCommand::Commit => {
                    // call commit_changes
                    commit(&args[0]);
                }
                DvcsCommand::Remove => {
                    // call remove_file
                    remove(&args[0]);
                }
                DvcsCommand::Log => {
                    // call show_commit_logs
                    log();
                }
                DvcsCommand::Checkout => {
                    // call checkout_commit
                    checkout(&args[0]);
                }
                DvcsCommand::Status => {
                    // call show_status
                    status();
                }
                DvcsCommand::Help => {
                    print_help();
                }
                DvcsCommand::Exit => {
                    println!("Exiting DVCS...");
                    std::process::exit(0);
                }
                DvcsCommand::Push => {
                    // call push_changes
                    push();
                }
                DvcsCommand::Pull => {
                    // call pull_changes
                    pull();
                }
                DvcsCommand::Clone => {
                    // call clone_repository
                    clone(&args[0]);
                }

                // TO DO: add more commands as needed
            }
            Ok(())
        }

        /// This function is used to show the interface for the DVCS.
        pub fn show_interface() {
            loop {
                prompt_user();
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).unwrap();
                let user_input = user_input.trim();
                let mut args: Vec<String> = user_input.split_whitespace().map(|s| s.to_string()).collect();
                let command = parse_command(&args[0]);
                match command {
                    Ok(command) => {
                        args.remove(0);
                        let result = validate_command(&command, &args);
                        match result {
                            Ok(_) => {
                                let result = execute_command(&command, &args);
                                match result {
                                    Ok(_) => {}
                                    Err(e) => {
                                        handle_error(&e);
                                    }
                                }
                            }
                            Err(e) => {
                                handle_error(&e);
                            }
                        }
                    }
                    Err(e) => {
                        handle_error(&e);
                    }
                }
            }
        }

        /// This function is used to handle the errors using the Display trait implemented in user_hiding::syntax_checker_error_handling.
        fn handle_error(error: &DvcsError) {
            println!("{}", error);
        }


    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // Test ID: 1
    #[test]
    fn test_execute_command_success() {
        // TO DO: add test cases

        let command = DvcsCommand::Init;
        let args = vec![];
        let result = execute_command(&command, &args);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 2
    #[test]
    fn test_execute_command_failure() {
        // TO DO: add test cases

        let command = DvcsCommand::Add;
        let args = vec![];
        let result = execute_command(&command, &args);
        assert_eq!(result, Err(DvcsError::InvalidNumberOfArguments));

        let command = DvcsCommand::Commit;
        let args = vec![String::from("invalidArgs")];
        let result = execute_command(&command, &args);
        assert_eq!(result, Err(DvcsError::InvalidArguments));

    }

    // Test ID: 3
    #[test]
    fn test_show_interface() {
        // TO DO: add test cases

        let result = show_interface();
        assert_eq!(result, Ok(()));
    }

    // Test ID: 4
    #[test]
    fn test_handle_error() {
        // TO DO: add test cases

        let error = DvcsError::InvalidNumberOfArguments;
        let result = handle_error(&error);
        assert_eq!(result, Ok(()));
    }
}