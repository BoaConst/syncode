
/// FOR INCREMENT 2 OR ABOVE - IGNORE FOR NOW

mod authentication_manager {
    impl AuthenticationManager {

        use crate::syncode::user_hiding::*;      // including DvcsAuthError, DvcsAuthConfig, DvcsAuthLog
        use crate::syncode::repository_hiding::*;

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
        pub fn check_user_authorization(username: &str, repository: &Repository, command: &DvcsCommand) -> Result<(), DvcsAuthError> {
            // TO DO: implement this function

            // 1. Check if the username is the same as the username in the repository config file. 
                // If not, return Err(DvcsAuthError::InvalidUsername)
            // 2. Check if the user has the permission to perform the action on the repository. 
                // If not, return Err(DvcsAuthError::InvalidPermission)
            // 3. If both are passed, return Ok(()), else return Err(DvcsAuthError::InvalidUsername) or Err(DvcsAuthError::InvalidPassword)

            Ok(())
        }

        // Implementing the get_user_permissions function
        fn get_user_permissions(username: &str, repository: &Repository) -> Vec<String> {
            // TO DO: implement this function

            // 1. Get the user permissions from the repository config file
            // 2. Return the user permissions
            
            Ok(())
        }

        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syncode::repository_hiding::*;

    // Test ID: 1
    #[test]
    fn test_check_user_authorization_UsernameNotFound() {
        // Test if the function returns Err(DvcsAuthError::InvalidUsername) when the username is not found in the repository config file

        // TO DO: implement this test

        // 1. Create a repository config file with a username
        let repo = Repository::new("test_repo");
        let mut repo_config = RepositoryConfig::new(&repo);
        repo_config.set_username("test_user");

        // 2. Call the check_user_authorization function with a different username
        assert_eq!(check_user_authorization("test_user2", &repo, &DvcsCommand::Add), Err(DvcsAuthError::InvalidUsername));
    }

    // Test ID: 2
    #[test]
    fn test_check_user_auth_UsernameFound() {
        // Test if the function returns Ok(()) when the username is found in the repository config file

        // TO DO: implement this test

        // 1. Create a repository config file with a username
        let repo = Repository::new("test_repo");
        let mut repo_config = RepositoryConfig::new(&repo);
        repo_config.set_username("test_user");

        // 2. Call the check_user_authorization function with the same username
        assert_eq!(check_user_authorization("test_user", &repo, &DvcsCommand::Help), Ok(()));
    }


    // Test ID: 3
    #[test]
    fn test_get_user_perm_NotFound() {
        // TO DO: implement this test
        let repo = Repository::new("test_repo");
        let mut repo_config = RepositoryConfig::new(&repo);
        repo_config.set_username("test_user");
        repo_config.set_permission("edit");

        assert_eq!(get_user_permissions("test_user2", &repo), Err(DvcsAuthError::InvalidUsername));
    }

    // Test ID: 4
    #[test]
    fn test_get_user_perm_Found() {
        // TO DO: implement this test
        let repo = Repository::new("test_repo");
        let mut repo_config = RepositoryConfig::new(&repo);
        repo_config.set_username("test_user");
        repo_config.set_permission("edit");

        assert_eq!(get_user_permissions("test_user", &repo), Ok("edit"));
    }

    // Test ID: 5
    #[test]
    fn test_check_user_auth_PermissionFound() {
        // TO DO: implement this test
        let repo = Repository::new("test_repo");
        let mut repo_config = RepositoryConfig::new(&repo);
        repo_config.set_username("test_user");
        repo_config.set_permission("edit");

        assert_eq!(check_user_authorization("test_user", &repo, &DvcsCommand::Add), Ok(()));
    }

    // Test ID: 6
    #[test]
    fn test_check_user_auth_PermissionNotFound() {
        // TO DO: implement this test
        let repo = Repository::new("test_repo");
        let mut repo_config = RepositoryConfig::new(&repo);
        repo_config.set_username("test_user");
        repo_config.set_permission("viewer");

        assert_eq!(check_user_authorization("test_user", &repo, &DvcsCommand::Commit), Err(DvcsAuthError::InvalidPermission));
    }
}
