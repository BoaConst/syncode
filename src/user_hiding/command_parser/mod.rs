/*
# Module B.1 : Command Parser Module

**Designer:** Helia 
*/
// Importing modules
use super::repository_hiding::initialization::*;     // including DvcsCommand, DvcsError
use crate::machine_hiding;

/// This function is used to parse the DVCS command from the user input
pub fn parse_command(cmd_name: String) -> Result<DvcsCommand, DvcsError> {
    
    match &cmd_name[..] {
        "init" => Ok(DvcsCommand::Init),
        "add" => Ok(DvcsCommand::Add),
        "commit" => Ok(DvcsCommand::Commit),
        "remove" => Ok(DvcsCommand::Remove),
        "diff" => Ok(DvcsCommand::Diff),
        "log" => Ok(DvcsCommand::Log),
        "heads" => Ok(DvcsCommand::Heads),
        "cat" => Ok(DvcsCommand::Cat),
        "checkout" => Ok(DvcsCommand::Checkout),
        "status" => Ok(DvcsCommand::Status),
        "merge" => Ok(DvcsCommand::Merge),
        "push" => Ok(DvcsCommand::Push),
        "pull" => Ok(DvcsCommand::Pull),
        "clone" => Ok(DvcsCommand::Clone),
        // "help" => Ok(DvcsCommand::Help),
        _ => Err(DvcsError::InvalidCommand),
    }
}

/// This function is used to validate the DVCS command (making sure the necessary arguments are provided).
/// TO DO: add more validation methods as needed
pub fn validate_command(command: &DvcsCommand, args: Vec<&String>) -> Result<(), DvcsError> {
    match command {
        DvcsCommand::Init => {
            if args.len() != 1 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
            // check if the provided path exists
            let directory = args[0];
            println!("Path: {}", directory);
            if !machine_hiding::file_system_operations::check_path(directory) {
                return Err(DvcsError::InvalidPath);
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
        DvcsCommand::Heads => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Diff => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Cat => {
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
            if args.len() != 2 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Merge => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        DvcsCommand::Help => {
            if args.len() != 0 {
                return Err(DvcsError::InvalidNumberOfArguments);
            }
        }
        
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    // Test ID: 1
    // Test Purpose: To test the parse_command function on valid commands
    #[test]
    fn test_parse_command_valid_1() {
        let command = parse_command("init".to_string());
        assert_eq!(command, Ok(DvcsCommand::Init));
        let command = parse_command("add".to_string());
        assert_eq!(command, Ok(DvcsCommand::Add));
        let command = parse_command("commit".to_string());
        assert_eq!(command, Ok(DvcsCommand::Commit));
        let command = parse_command("merge".to_string());
        assert_eq!(command, Ok(DvcsCommand::Merge));
        let command = parse_command("cat".to_string());
        assert_eq!(command, Ok(DvcsCommand::Cat));

    }
    // Test ID: 2
    #[test]
    fn test_parse_command_valid_2() {
        let command = parse_command("remove".to_string());
        assert_eq!(command, Ok(DvcsCommand::Remove));
        let command = parse_command("log".to_string());
        assert_eq!(command, Ok(DvcsCommand::Log));
        let command = parse_command("checkout".to_string());
        assert_eq!(command, Ok(DvcsCommand::Checkout));
        let command = parse_command("heads".to_string());
        assert_eq!(command, Ok(DvcsCommand::Heads));
        let command = parse_command("diff".to_string());
        assert_eq!(command, Ok(DvcsCommand::Diff));

    }
    // Test ID: 3
    #[test]
    fn test_parse_command_valid_3() {
        let command = parse_command("status".to_string());
        assert_eq!(command, Ok(DvcsCommand::Status));
        let command = parse_command("push".to_string());
        assert_eq!(command, Ok(DvcsCommand::Push));
        let command = parse_command("pull".to_string());
        assert_eq!(command, Ok(DvcsCommand::Pull));
        let command = parse_command("clone".to_string());
        assert_eq!(command, Ok(DvcsCommand::Clone));
        let command = parse_command("help".to_string());
        assert_eq!(command, Ok(DvcsCommand::Help));

    }

    // Test ID: 4
    // Test Purpose: To test the validate_command function on valid commands
    #[test]
    fn test_validate_command_valid() {
        let command = parse_command("init".to_string());
        let args = vec![];
        let result = validate_command(&command.unwrap(), args.iter().collect());
        assert_eq!(result, Ok(()));
    
        let command = parse_command("add".to_string());
        let args = vec!["file1.txt".to_string()];
        let result = validate_command(&command.unwrap(), args.iter().collect());
        assert_eq!(result, Ok(()));
    }

    // Test ID: 5
    // Test Purpose: To test the validate_command function on invalid arguments
    #[test]
    fn test_validate_command_invalid() {
        let command = parse_command("help".to_string());
        let args = vec!["file1.txt".to_string()];
        let result = validate_command(&command.unwrap(), args.iter().collect());
        assert_eq!(result, Err(DvcsError::InvalidNumberOfArguments));
    
        let command = parse_command("remove".to_string());
        let args = vec![];
        let result = validate_command(&command.unwrap(), args.iter().collect());
        assert_eq!(result, Err(DvcsError::InvalidNumberOfArguments));
    }

    // Test ID: 6
    // Test Purpose: To test the parse_command function on invalid commands
    #[test]
    fn test_parse_command_invalid() {
        let command = parse_command("invalid".to_string());
        assert_eq!(command, Err(DvcsError::InvalidCommand));

        let command = parse_command("invalid234".to_string());
        assert_eq!(command, Err(DvcsError::InvalidCommand));
    }

}
