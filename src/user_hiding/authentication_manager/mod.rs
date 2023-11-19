
/// FOR INCREMENT 2 OR ABOVE - IGNORE FOR NOW


pub struct authentication_manager {

    use crate::syncode::user_hiding::*;      // including DvcsAuthError, DvcsAuthConfig, DvcsAuthLog

    // Implementing the Display trait for DvcsAuthError
    impl fmt::Display for DvcsAuthError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {

                DvcsAuthError::InvalidUsername => write!(f, "Invalid username"),
                DvcsAuthError::InvalidPassword => write!(f, "Invalid password"),
                DvcsAuthError::InvalidEmail => write!(f, "Invalid email"),

                // TO DO: add more error types as needed
            }
        }
    }
    
    // Implementing the check_user_authentication function
    pub fn check_user_authorization(username: &str, repository: &str, action: &str) -> Result<(), DvcsAuthError> {
        // TO DO: implement this function

        // 1. Check if the username is the same as the username in the repository config file. 
            // If not, return Err(DvcsAuthError::InvalidUsername)
        // 2. Check if the user has the permission to perform the action on the repository. 
            // If not, return Err(DvcsAuthError::InvalidPermission)
        // 3. If both are passed, return Ok(()), else return Err(DvcsAuthError::InvalidUsername) or Err(DvcsAuthError::InvalidPassword)

        Ok(())
    }

    // Implementing the report_user_permissions function
    pub fn get_user_permissions(username: &str, repository: &str) -> Vec<String> {
        // TO DO: implement this function

        // 1. Get the user permissions from the repository config file
        // 2. Return the user permissions
        
        Ok(())
    }

    
}