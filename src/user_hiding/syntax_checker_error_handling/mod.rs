

// Importing modules
use std::error::Error;
use std::fmt::{self, write};
use super::repository_hiding::initialization::*;      // including DvcsCommand, DvcsError

// Implementing the Display trait for DvcsError 
impl fmt::Display for DvcsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DvcsError::InvalidCommand => write!(f, "Invalid command"),
            DvcsError::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            DvcsError::InvalidArguments => write!(f, "Invalid arguments"),
            _ => write!(f, "To be implemented"),
            // TO DO: add more error types as needed
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test ID: 1
    #[test]
    fn test_error_message_InvalidCommand() {
        // TO DO: add test cases as needed
        let err = DvcsError::InvalidCommand;
        assert_eq!(err.to_string(), "Invalid command");
    }

    // Test ID: 2
    #[test]
    fn test_error_message_InvalidNumberOfArguments() {
        let err = DvcsError::InvalidNumberOfArguments;
        assert_eq!(err.to_string(), "Invalid number of arguments");
    }

    // Test ID: 3
    #[test]
    fn test_error_message_InvalidArguments() {
        let err = DvcsError::InvalidArguments;
        assert_eq!(err.to_string(), "Invalid arguments");

    }
}