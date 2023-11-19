pub struct syntax_checker_error_handling {

    // Importing modules
    use std::error::Error;
    use std::fmt;
    use crate::syncode::repository_hiding::*;      // including DvcsCommand, DvcsError

    // Implementing the Display trait for DvcsError 
    impl fmt::Display for DvcsError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DvcsError::InvalidCommand => write!(f, "Invalid command"),
                DvcsError::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
                DvcsError::InvalidArguments => write!(f, "Invalid arguments"),

                // TO DO: add more error types as needed
            }
        }
    }

}
