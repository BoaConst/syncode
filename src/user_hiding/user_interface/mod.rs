pub struct user_interface {

    // Importing modules
    use std::io;
    use std::io::Write;
    use crate::syncode::user_hiding::error_handling_module::*;
    use crate::syncode::user_hiding::command_parser::*;
    use crate::syncode::repository_hiding::*;      // including DvcsCommand, DvcsError
    // use crate::dvcs::dvcs_auth::*;

    /// This function is used to print the DVCS prompt.
    pub fn prompt_user() {
        print!("dvcs> ");
        io::stdout().flush().unwrap();
    }

    /// This function is used to read the user input and call the appropriate functions to process the input and/or execute commands from other modules.
    pub fn show_user_interface() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()

        // CALL METHODS FROM COMMMAND PARSER MODULE HERE TO VALIDATE/PROCESS THE INPUT
        // call parse_command 
        let command = parse_command(&input);
        match command {
            Ok(command) => {
                // call validate_command
                match validate_command(&command) {
                    Ok(_) => {
                        // call execute_command
                        match execute_command(&command, &args) {
                            Ok(_) => {
                                // call command from repository module
                                match command {
                                    DvcsCommand::Init => {
                                        // call init_repository
                                        match init() {
                                            Ok(_) => {
                                                // print success message
                                                println!("Repository initialized successfully");
                                                // call prompt_user
                                                prompt_user();
                                                
                                            }
                                            Err(e) => {
                                                // print error message
                                                println!("{}", e);
                                                // call prompt_user
                                                prompt_user();
                                            }
                                        }
                                    }
                                    DvcsCommand::Add => {
                                        // call add_file
                                        match add(&args[0]) {
                                            Ok(_) => {
                                                // print success message
                                                println!("File added successfully");
                                                // call prompt_user
                                                prompt_user();
                                            }
                                            Err(e) => {
                                                // print error message
                                                println!("{}", e);
                                                // call prompt_user
                                                prompt_user();
                                            }
                                        }
                                    }
                                    DvcsCommand::Commit => {
                                        // call commit_changes
                                        match commit(&args[0]) {
                                            Ok(_) => {
                                                // print success message
                                                println!("Changes committed successfully");
                                                // call prompt_user
                                                prompt_user();
                                            }
                                            Err(e) => {
                                                // print error message
                                                println!("{}", e);
                                                // call prompt_user
                                                prompt_user();
                                            }
                                        }
                                    }
                                    DvcsCommand::Remove => {
                                        // call remove_file
                                        match remove(&args[0]) {
                                            Ok(_) => {
                                                // print success message
                                                println!("File removed successfully");
                                                // call prompt_user
                                                prompt_user();
                                            }
                                            Err(e) => {
                                                // print error message
                                                println!("{}", e);
                                                // call prompt_user
                                                prompt_user();
                                            }
                                        }
                                    }
                                    DvcsCommand::Log => {
                                        // call show_commit_logs
                                        match log() {
                                            Ok(_) => {
                                                // print success message
                                                println!("Commit logs shown successfully");
                                                // call prompt_user
                                                prompt_user();
                                            }
                                            Err(e) => {
                                                // print error message
                                                println!("{}", e);
                                                // call prompt_user
                                                prompt_user();
                                            }
                                        }
                                    }
                                    DvcsCommand::Checkout => {
                                        // call checkout_commit
                                        match checkout(&args[0]) {
                                            Ok(_) => {
                                                // print success message
                                                println!("Commit checked out successfully");
                                                // call prompt_user
                                                prompt_user();
                                            }
                                            Err(e) => {
                                                // print error message
                                                println!("{}", e);
                                                // call prompt_user
                                                prompt_user();
                                            }
                                        }
                                    }
                                    DvcsCommand::Help => {
                                        // call print_help
                                        print_help();
                                        // call prompt_user
                                        prompt_user();
                                    }
                                    DvcsCommand::Quit => {
                                        // print success message
                                        println!("Exiting DVCS...");
                                        // exit the program
                                        std::process::exit(0);
                                    }

                                    // TO DO: add/edit more commands as needed
                                }

                            }
                            Err(e) => {
                                // print error message
                                println!("{}", e);
                                // call prompt_user
                                prompt_user();
                            }
                        }
                    }
                    Err(e) => {
                        // print error message
                        println!("{}", e);
                        // call prompt_user
                        prompt_user();
                    }
                }
            }
            Err(e) => {
                // print error message
                println!("{}", e);
                // call prompt_user
                prompt_user();
            }
        }
    }

    /// This function is used to print the DVCS help message.
    pub fn print_help() {
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
    pub fn execute_command(command: &DvcsCommand, args: &Vec<String>) -> Result<(), DvcsError> {
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

            // TO DO: add more commands as needed
        }
        Ok(())
    }

}