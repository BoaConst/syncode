pub struct command_parser {

    // Importing modules
    use crate::syncode::repository_hiding::*;      // including DvcsCommand, DvcsError

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

    /* 2nd increment only - disregard now
    /// This function is used to parse the DVCS authentication command.
    pub fn parse_auth_command(command: &str) -> Result<DvcsAuthCommand, DvcsAuthError> {
        match command {
            "register" => Ok(DvcsAuthCommand::Register),
            "login" => Ok(DvcsAuthCommand::Login),
            "logout" => Ok(DvcsAuthCommand::Logout),
            "add" => Ok(DvcsAuthCommand::Add),
            "remove" => Ok(DvcsAuthCommand::Remove),
            "list" => Ok(DvcsAuthCommand::List),
            _ => Err(DvcAuthError::InvalidCommand),
        }
    } 
    */

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
}