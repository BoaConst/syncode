
// Importing modules
use super::repository_hiding::initialization::*;     // including DvcsCommand, DvcsError
use std::env;
use super::machine_hiding::file_system_operations::*;
/// This function is used to parse the DVCS command from the user input
pub fn parse_command(user_input: &str) -> Result<DvcsCommand, DvcsError> {
    let command = user_input.split_whitespace().next().unwrap();
    match command {
        "init" => Ok(DvcsCommand::Init),
        "add" => Ok(DvcsCommand::Add),
        "commit" => Ok(DvcsCommand::Commit),
        "remove" => Ok(DvcsCommand::Remove),
        "log" => Ok(DvcsCommand::Log),
        "checkout" => Ok(DvcsCommand::Checkout),
        "status" => Ok(DvcsCommand::Status),
        "push" => Ok(DvcsCommand::Push),
        "pull" => Ok(DvcsCommand::Pull),
        "clone" => Ok(DvcsCommand::Clone),
        "help" => Ok(DvcsCommand::Help),
        "exit" => Ok(DvcsCommand::Exit),
        _ => Err(DvcsError::InvalidCommand),
    }
}

/// This function is used to validate the DVCS command (making sure the necessary arguments are provided).
/// TO DO: add more validation methods as needed
pub fn validate_command(command: &DvcsCommand, args: &[String]) -> Result<(), DvcsError> {
    match command {
        DvcsCommand::Init => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Add => {
            if args.len() != 1 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Commit => {
            if args.len() != 1 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Remove => {
            if args.len() != 1 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Log => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Checkout => {
            if args.len() != 1 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Status => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Push => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Pull => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Clone => {
            if args.len() != 1 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Help => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Exit => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
    }
    Ok(())
}


// For prototype:

use super::machine_hiding;
use super::repository_hiding;

pub fn command(cmd_name: String, args: Vec<&String>) {
    let cwd = machine_hiding::file_system_operations::get_cwd();
    match &cmd_name[..] {
        "init" => {
            let repo_root_path = &args[0];
            repository_hiding::initialization::init(repo_root_path);
        },
        "clone" => {
            todo!()
        },
        "add" => {
            let file_abs_path = machine_hiding::file_system_operations::join_paths(&cwd, &args[0]);
            let repo_root_path = machine_hiding::file_system_operations::find_repo_root_path(&file_abs_path);
            println!("repo find at {}", repo_root_path);
            let file_rel_path = machine_hiding::file_system_operations::find_rel_path(&repo_root_path, &file_abs_path);
            println!("file rel path find at {}", file_rel_path);
            let mut r = repository_hiding::initialization::open(&repo_root_path);
            r.add_file(&file_rel_path);
            r.save();

            println!("{}", r);
        },
        _ => println!("unknown command: {}", cmd_name),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test ID: 1
    // Test Purpose: To test the parse_command function on valid commands
    #[test]
    fn test_parse_command_valid_1() {
        let command = parse_command("init");
        assert_eq!(command, Ok(DvcsCommand::Init));
        let command = parse_command("add");
        assert_eq!(command, Ok(DvcsCommand::Add));
        let command = parse_command("commit");
        assert_eq!(command, Ok(DvcsCommand::Commit));

    }
    // Test ID: 2
    #[test]
    fn test_parse_command_valid_2() {
        let command = parse_command("remove");
        assert_eq!(command, Ok(DvcsCommand::Remove));
        let command = parse_command("log");
        assert_eq!(command, Ok(DvcsCommand::Log));
        let command = parse_command("checkout");
        assert_eq!(command, Ok(DvcsCommand::Checkout));

    }
    // Test ID: 3
    #[test]
    fn test_parse_command_valid_3() {
        let command = parse_command("status");
        assert_eq!(command, Ok(DvcsCommand::Status));
        let command = parse_command("push");
        assert_eq!(command, Ok(DvcsCommand::Push));
        let command = parse_command("pull");
        assert_eq!(command, Ok(DvcsCommand::Pull));
        let command = parse_command("clone");
        assert_eq!(command, Ok(DvcsCommand::Clone));
        let command = parse_command("help");
        assert_eq!(command, Ok(DvcsCommand::Help));
        let command = parse_command("exit");
        assert_eq!(command, Ok(DvcsCommand::Exit));
    }

    // Test ID: 4
    // Test Purpose: To test the validate_command function on valid commands
    #[test]
    fn test_validate_command_valid() {
        let command = parse_command("init");
        let args = vec![];
        let result = validate_command(&command.unwrap(), &args);
        assert_eq!(result, Ok(()));
    
        let command = parse_command("add");
        let args = vec!["file1.txt".to_string()];
        let result = validate_command(&command.unwrap(), &args);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 5
    // Test Purpose: To test the validate_command function on invalid arguments
    #[test]
    fn test_validate_command_invalid() {
        let command = parse_command("help");
        let args = vec!["file1.txt".to_string()];
        let result = validate_command(&command.unwrap(), &args);
        assert_eq!(result, Err(DvcsError::InvalidNumberOfArguments));
    
        let command = parse_command("remove");
        let args = vec![];
        let result = validate_command(&command.unwrap(), &args);
        assert_eq!(result, Err(DvcsError::InvalidNumberOfArguments));
    }

    // Test ID: 6
    // Test Purpose: To test the parse_command function on invalid commands
    #[test]
    fn test_parse_command_invalid() {
        let command = parse_command("invalid");
        assert_eq!(command, Err(DvcsError::InvalidCommand));

        let command = parse_command("invalid234");
        assert_eq!(command, Err(DvcsError::InvalidCommand));
    }

}