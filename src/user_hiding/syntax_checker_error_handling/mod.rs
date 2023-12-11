/*
# Module B.2 : Error Handling Module

**Designer:** Helia 
*/

// Importing modules
use std::error::Error;
use std::fmt::{self, write};
use super::repository_hiding::initialization::*;      // including DvcsCommand, DvcsError

// Implementing the Display trait for DvcsError 
impl fmt::Display for DvcsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // These are errors related to the DVCS initialization commands (init, clone)
            DvcsError::InvalidCommand => write!(f, "Invalid command"),
            DvcsError::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            DvcsError::InvalidArguments => write!(f, "Invalid arguments"),
            DvcsError::InvalidPath => write!(f, "Invalid path"),
            DvcsError::DuplicateRepositoryError => write!(f, "Duplicated repository name"),
            DvcsError::CopyFilesError => write!(f, "Error copying files"),
            DvcsError::CloneError => write!(f, "Error cloning repository"),
            DvcsError::InvalidMetadataError => write!(f, "Unable to read metadata for the repository"),

            // TODO: add more error types and/or fix the error messages as needed
            DvcsError::AddError => write!(f, "Error adding files"),
            DvcsError::CommitError => write!(f, "Error committing files"),
            DvcsError::PushError => write!(f, "Error pushing files"),
            DvcsError::PullError => write!(f, "Error pulling files"),
            DvcsError::MergeError => write!(f, "Error merging files"),
            DvcsError::CheckoutError => write!(f, "Error checking out files"),
            DvcsError::RemoveError => write!(f, "Error removing files"),
            DvcsError::LogError => write!(f, "Error logging files"),
            DvcsError::StatusError => write!(f, "Error checking status of files"),
            DvcsError::CatError => write!(f, "Error catting files"),
            DvcsError::DiffError => write!(f, "Error diffing files"),

            _ => write!(f, "To be implemented"),
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
