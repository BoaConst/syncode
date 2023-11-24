/*
# Module C.1 : Initialization Module

**Designer:** Helia | **Reviewer:** Shunzhi
*/

pub mod init_module {
    
    impl InitializationModule {

        // Importing modules
        use std::io;
        use std::io::Write;
        use crate::syncode::repository_hiding::*;      // including DvcsError, Repository, SyncodeDirectory

        // implement Repository
        impl Repository {
            pub fn new(repository_name: &str) -> Repository {

                Repository {
                    name: repository_name.to_string(),
                    path: format!("./{}", repository_name),
                }

            }

            pub fn clone(&self, url: &str) {
                // TO DO: implement this function
                // 1. Clone the existing repository to the new repository

            }
        }

        // implement SyncodeDirectory
        impl SyncodeDirectory {
            pub fn new(repository_name: &str) -> SyncodeDirectory {

                SyncodeDirectory {
                    name: format!(".syncode/{}", repository_name),
                    url: format!(""),
                    files: Vec::new(),
                    commits: Vec::new(),
                    branches: Vec::new(),
                }

                // TO DO: how to actually create the .syncode directory???

            }

            pub fn get_repository_name(url: &str) -> String {
                // TO DO: implement this function
                // 1. Get the repository name from the url
                // 2. Return the repository name
            }
        }



        pub fn init(repository_name: &str) -> Result<(), DvcsError::InitError> {
            // TO DO: implement this function

            // 1. Create a new repository
            let repository = Repository::new(repository_name);

            // 2. Create a new .syncode directory
            let syncode_directory = SyncodeDirectory::new(repository_name);

            Ok(())
        }

        pub fn clone(url: &str) -> Result<(), DvcsError::CloneError> {
            // TO DO: implement this function

            // 1. Create a new repository
            let repository_name = SyncodeDirectory::get_repository_name(url);
            let repository = Repository::new(repository_name);

            // 2. Clone the existing repository to the new repository
            repository.clone(url);

            Ok(())
        }

    }


}

/// Test cases
#[cfg(test)]
mod tests {
    use super::*;
    use crate::syncode::repository_hiding::init_module::*;

    // Test ID: 1
    #[test]
    fn test_init_successful() {
        let repository_name = "test_repository";
        let result = InitializationModule::init(repository_name);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 2
    #[test]
    fn test_init_duplicate() {
        let repository_name = "test_repository";
        let duplicated_repository_name = "test_repository";
        let result = InitializationModule::init(repository_name);
        let result2 = InitializationModule::init(duplicated_repository_name);
        assert_eq!(result2, Err(DvcsError::InitError::DuplicateRepositoryError));
    }

    // Test ID: 3
    #[test]
    fn test_init_invalid() {
        let invalid_repository_name = "invalid_repository";
        let result = InitializationModule::init(invalid_repository_name);
        assert_eq!(result, Err(DvcsError::InitError::InvalidRepositoryNameError));
    }

    // Test ID: 4
    #[test]
    fn test_clone_successful() {
        let url = "https://example.com/repo.dvcs";
        let result = InitializationModule::clone(url);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 5
    #[test]
    fn test_clone_invalid_url() {
        let invalid_url = "https://invalid.invalid";
        let result = InitializationModule::clone(invalid_url);
        assert_eq!(result, Err(DvcsError::CloneError::InvalidUrlError));
    }

    // Test ID: 6
    #[test]
    fn test_clone_network_error() {
        let network_error_url = "https://network.error";
        let result = InitializationModule::clone(network_error_url);
        assert_eq!(result, Err(DvcsError::CloneError::NetworkError));
    }       


}