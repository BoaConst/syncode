/*
# Module C.1 : Initialization Module

**Designer:** Helia | **Reviewer:** Shunzhi
*/

use std::fs;
use std::path::Path;
use crate::machine_hiding;
/// This Repository struct is used to represent the local DVCS repository to be created by init and clone commands.
pub struct Repository {

    // repository name
    name: String,
    // repository path
    path: String,
    // repository files
    files: Vec<String>,
    // repository commits
    commits: Vec<String>,

    branches: Vec<String>,

    // TO DO: add more fields as needed
}  

// @todo: implement this as used in user_hiding::authentication_manager
pub struct RepositoryConfig {}


/// This enum type is used to represent the DVCS functionalities.
/// used in the CommandParser module to parse the user input and in the interface module to execute the DVCS commands.
#[derive(Debug, PartialEq)]
pub enum DvcsCommand {
    Init,
    Add,
    Commit,
    Remove,
    Log,
    Checkout,
    Status,
    Push,
    Pull,
    Clone,
    Help,
    Exit,
}


/// This struct is used to represent the DVCS error.
/// There can be many types of errors inside DvcsError such as invalid command, invalid number of arguments, etc.
#[derive(Debug, PartialEq)]
pub enum DvcsError {
    /// This error type is used to report errors related to invalid command.
    InvalidCommand,
    /// This error type is used to report errors related to invalid number of arguments.
    InvalidNumberOfArguments,
    /// This error type is used to report errors related to invalid arguments.
    InvalidArguments,
    /// This error type is used to report errors related to initializing the DVCS.
    // InitError,
    /// This error type is used to report errors related to adding files to the DVCS.
    AddError,
    /// This error type is used to report errors related to committing changes to the DVCS.
    CommitError,
    /// This is error for clone 
    CloneError,

    PushError,
    PullError,
    MergeError,
    DefaultError,

    // TO DO: add more error types as needed
}


// Importing modules
use std::io;
use std::io::Write;

use serde_json;
use serde::{Serialize, Deserialize};

// implement Repository
impl Repository {
    pub fn new(repository_name: &str) -> Repository {

        Repository {
            name: repository_name.to_string(),
            path: format!("{}", repository_name),
            files: Vec::new(),
            commits: Vec::new(),
            branches: Vec::new(),
        }

    }

    pub fn clone(&self, url: &str) {
        // TO DO: implement this function
        // 1. Clone the existing repository to the new repository

    }

    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.name).unwrap();
        let dev_path = machine_hiding::file_system_operations::join_paths(&self.name, &".dvcs".to_string());
        machine_hiding::file_system_operations::write_string(&dev_path, &String::from("repo.json"), &serialized);
    }
}




pub fn init(repository_name: &str) -> Result<(), DvcsError> {
    // TO DO: implement this function
    if !machine_hiding::file_system_operations::check_path(&repository_name.to_string()) {
        panic!("path not exist")
    }

    let dev_path = machine_hiding::file_system_operations::join_paths(&repository_name.to_string(), &".dvcs".to_string());
    assert!(!machine_hiding::file_system_operations::check_path(&dev_path), "Repo already initialized!");
    machine_hiding::file_system_operations::create_dir_all(&dev_path);

    // 1. Create a new repository
    let repository = Repository::new(repository_name);
    repository.save();
    println!("Initialized repo @ {}", repository_name);
    Ok(())
}

pub fn clone(repository_name: &str, url: &str) -> Result<(), DvcsError> {
    // TO DO: implement this function

    // 1. Create a new repository
    let repository = Repository::new(repository_name);

    // 2. Clone the existing repository to the new repository
    repository.clone(url);

    Ok(())
}

 

/// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    // Test ID: 1
    #[test]
    fn test_init_successful() {
        let repository_name = "test_repository";
        let result = init(repository_name);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 2
    #[test]
    fn test_init_duplicate() {
        let repository_name = "test_repository";
        let duplicated_repository_name = "test_repository";
        let result = init(repository_name);
        let result2 = init(duplicated_repository_name);
        // assert_eq!(result2, Err(DvcsError::InitError::DuplicateRepositoryError));
        assert_eq!(result2, Err(DvcsError::DefaultError));
    }

    // Test ID: 3
    #[test]
    fn test_init_invalid() {
        let invalid_repository_name = "invalid_repository";
        let result = init(invalid_repository_name);
        // assert_eq!(result, Err(DvcsError::InitError::InvalidRepositoryNameError));
        assert_eq!(result, Err(DvcsError::DefaultError));
    }

    // Test ID: 4
    #[test]
    fn test_clone_successful() {
        let url = "https://example.com/repo.dvcs";
        let repository_name = "test_repository";
        let result = clone(repository_name, url);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 5
    #[test]
    fn test_clone_invalid_url() {
        let invalid_url = "https://invalid.invalid";
        let repository_name = "test_repository";
        let result = clone(repository_name, invalid_url);
        // assert_eq!(result, Err(DvcsError::CloneError::InvalidUrlError));
        assert_eq!(result, Err(DvcsError::DefaultError));
    }

    // Test ID: 6
    #[test]
    fn test_clone_network_error() {
        let network_error_url = "https://network.error";
        let repository_name = "test_repository";
        let result = clone(repository_name, network_error_url);
        // assert_eq!(result, Err(DvcsError::CloneError::NetworkError));
        assert_eq!(result, Err(DvcsError::DefaultError));
    }       


}