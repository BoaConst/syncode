/*
# Module C.1 : Initialization Module

**Designer:** Helia 
*/
// Importing modules
use std::io;
use std::io::Write;
use serde_json::json;
use serde_json;
use serde::{Serialize, Deserialize};
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

}  

// @todo: implement this as used in user_hiding::authentication_manager
pub struct RepositoryConfig {}


/// This enum type is used to represent the DVCS functionalities.
/// used in the CommandParser module to parse the user input and in the interface module to execute the DVCS commands.
#[derive(Debug, PartialEq, Clone)]
pub enum DvcsCommand {
    Init,
    Add,
    Commit,
    Remove,
    Log,
    Checkout,
    Status,
    Heads,
    Cat,
    Diff,
    Merge,
    Push,
    Pull,
    Clone,
    Help,
}


/// This struct is used to represent the DVCS error.
/// There can be many types of errors inside DvcsError such as invalid command, invalid number of arguments, etc.
#[derive(Debug, PartialEq, Clone)]
pub enum DvcsError {
    /// This error type is used to report errors related to initialization commands (init and clone)
    InvalidCommand,
    InvalidNumberOfArguments,
    InvalidArguments,
    InvalidPath,
    DuplicateRepositoryError,
    CopyFilesError,
    InvalidMetadataError,
    CloneError,
    
    // These are errors related to the DVCS commands
    AddError,
    CommitError, 
    PushError,
    PullError,
    MergeError,
    CheckoutError,
    RemoveError,
    LogError,
    StatusError,
    CatError,
    DiffError,
    
    // TODO: add more error types as needed
}

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

    pub fn clone(&self, source_path: &str) -> Result<(), DvcsError> {

        // 1. Check if the source repository exists
        if !machine_hiding::file_system_operations::check_path(&source_path.to_string()) {
            println!("Source_path: {}", source_path);
            return Err(DvcsError::InvalidPath);
        }

        // 2. Copy files from the source repository to the new repository
        match self.copy_repository_contents(source_path) {
            Ok(_) => {},
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn save(&mut self) {
        // let serialized = serde_json::to_string(&self.name).unwrap();
        // let dev_path = machine_hiding::file_system_operations::join_paths(&self.name, &".dvcs".to_string());
        // machine_hiding::file_system_operations::write_string(&dev_path, &String::from("repo.json"), &serialized);
        let dev_path = machine_hiding::file_system_operations::join_paths(&self.name, &".dvcs".to_string());

        // Create a serde_json::Value representing the entire struct
        let repo_json = json!({
            "name": &self.name,
            "path": &self.path,
            "files": &self.files,
            "commits": &self.commits,
            "branches": &self.branches,
            // Add other fields as needed
        });

        // Serialize and write the entire struct to "repo.json"
        let serialized = serde_json::to_string_pretty(&repo_json).unwrap();
        machine_hiding::file_system_operations::write_string(&dev_path, &String::from("repo.json"), &serialized);

    }

    // Function to copy files from the source repository to the destination repository
    fn copy_repository_contents(&self, source_path: &str) -> Result<(), DvcsError> {
        // Create a new repository object for the source repository
        let mut src_repository = Repository::new(source_path);
        src_repository.read_metadata();

        // Iterate over files in the source repository and copy them to the new repository
        for file in &src_repository.files {
            let source_file_path = machine_hiding::file_system_operations::join_paths(&source_path.to_string(), file);
            
            let target_file_path = machine_hiding::file_system_operations::join_paths(&self.name, file);

            match machine_hiding::file_system_operations::copy_file(&source_file_path, &target_file_path) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Failed to copy file {}: {}", file, err);
                    return Err(DvcsError::CopyFilesError);
                }
            }
            
        }

        Ok(())
    }

    // Function to read metadata from .dvcs/repo.json into the Repository struct
    pub fn read_metadata(&mut self) -> Result<(), DvcsError> {
        let dev_path = machine_hiding::file_system_operations::join_paths(&self.name, &".dvcs/repo.json".to_string());

        // Read the content of "repo.json" file
        if let Ok(metadata) = machine_hiding::file_system_operations::read_string(&dev_path) {
            // Deserialize the content into a serde_json::Value
            if let Ok(repo_json) = serde_json::from_str::<serde_json::Value>(&metadata) {
                
                // Extract the "files" array and update the Repository struct
                if let Some(files_json) = repo_json["files"].as_array() {
                    self.files = files_json.iter()
                        .filter_map(|value| value.as_str())
                        .map(String::from)
                        .collect();
                }

                // Extract the "commits" array and update the Repository struct
                if let Some(commits_json) = repo_json["commits"].as_array() {
                    self.commits = commits_json.iter()
                        .filter_map(|value| value.as_str())
                        .map(String::from)
                        .collect();
                }

                // Extract the "branches" array and update the Repository struct
                if let Some(branches_json) = repo_json["branches"].as_array() {
                    self.branches = branches_json.iter()
                        .filter_map(|value| value.as_str())
                        .map(String::from)
                        .collect();
                }

                Ok(())
            } else {
                println!("Path: {}", dev_path);
                Err(DvcsError::InvalidMetadataError)
            }
        } else {
            println!("Path: {}", dev_path);
            Err(DvcsError::InvalidPath)
        }
    }
}

pub fn init(repository_path: &str) -> Result<(), DvcsError> {

    // 1. Check if the repository already exists
    let dev_path = machine_hiding::file_system_operations::join_paths(&repository_path.to_string(), &".dvcs".to_string());
    if machine_hiding::file_system_operations::check_path(&dev_path) {
        return Err(DvcsError::DuplicateRepositoryError);
    }
    // 2. Create the repository directory
    machine_hiding::file_system_operations::create_dir_all(&dev_path);

    // 3. Create the Repository struct and repository files
    let mut repository = Repository::new(repository_path);
    repository.save();
    println!("Initialized repo @ {}", repository_path);
    Ok(())
}


pub fn clone(source_path: &str, destination: &str) -> Result<(), DvcsError> {
    // TO DO: implement this function
    println!("Cloning repo @ {}", source_path);

    // 1. Check if the source repository and destination repository exist
    if !machine_hiding::file_system_operations::check_path(&source_path.to_string()) {
        println!("Source_path: {}", source_path);
        return Err(DvcsError::InvalidPath);
    }
    if !machine_hiding::file_system_operations::check_path(&destination.to_string()) {
        println!("Destination: {}", destination);
        return Err(DvcsError::InvalidPath);
    }

    // 2. Create the destination repository directory
    let dev_path_dest = machine_hiding::file_system_operations::join_paths(&destination.to_string(), &".dvcs".to_string());
    machine_hiding::file_system_operations::create_dir_all(&dev_path_dest);

    // 3. Read metadata from the source repository
    let repo_json_path_src = machine_hiding::file_system_operations::join_paths(&source_path.to_string(), &".dvcs/repo.json".to_string());
    if !machine_hiding::file_system_operations::check_path(&repo_json_path_src) {
        println!("Path to repo.json metadata: {}", repo_json_path_src);
        return Err(DvcsError::InvalidPath);
    }
    let repo_json_content = match machine_hiding::file_system_operations::read_string(&repo_json_path_src) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading {}: {}", repo_json_path_src, err);
            return Err(DvcsError::CloneError);
        }
    };

    // Change the name and path to destination repository
    let mut repo_json = serde_json::from_str::<serde_json::Value>(&repo_json_content).unwrap();
    repo_json["name"] = serde_json::Value::String(destination.to_string());
    repo_json["path"] = serde_json::Value::String(destination.to_string());

    let content = serde_json::to_string_pretty(&repo_json).unwrap();

    // 4. Write metadata to the destination repository
    machine_hiding::file_system_operations::write_string(&dev_path_dest, &String::from("repo.json"), &content);

    // 5. Create the Repository struct and read metadata from the destination repository
    let mut repository = Repository::new(destination);
    repository.read_metadata();

    // 6. Clone the source repository to the new repository
    match repository.clone(source_path) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Failed to clone repository: {}", source_path);
            return Err(DvcsError::CloneError);
        }
    }

    println!("Cloned repo @ {}", destination);
    Ok(())
}



/// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    // Test ID: 1
    #[test]
    fn test_repository_new() {
        let repository = Repository::new("test_repo");
        assert_eq!(repository.name, "test_repo");
        assert_eq!(repository.path, "test_repo");
        assert!(repository.files.is_empty());
        assert!(repository.commits.is_empty());
        assert!(repository.branches.is_empty());
    }

    // Test ID: 2
    #[test]
    fn test_repository_clone_invalid_path() {
        let repository = Repository::new("destination_repo");
        let invalid_source_repository = "invalid_source_repo";

        // Attempt cloning from an invalid path
        let result = repository.clone(invalid_source_repository);

        // Assertions
        assert_eq!(result, Err(DvcsError::InvalidPath));
    }

    // Test ID: 3
    #[test]
    fn test_repository_save() {
        let cwd = machine_hiding::file_system_operations::get_cwd();
        let mut repository = Repository::new(&cwd);

        // Mock data
        repository.commits.push("commit1".to_string());
        repository.branches.push("main".to_string());

        // Perform save
        repository.save();

        // Assertions (additional: check if metadata is correctly written) 
        let dev_path = machine_hiding::file_system_operations::join_paths(&cwd.to_string(), &".dvcs/repo.json".to_string());
        let metadata_content = machine_hiding::file_system_operations::read_string(&dev_path).unwrap();

        // Change the name and path to destination repository
        let mut repo_json = serde_json::from_str::<serde_json::Value>(&metadata_content).unwrap();
        assert_eq!(repo_json["name"], serde_json::Value::String(cwd.to_string()));
        assert_eq!(repo_json["path"], serde_json::Value::String(cwd.to_string()));
        assert_eq!(repo_json["commits"], serde_json::Value::Array(vec![serde_json::Value::String("commit1".to_string())]));
        assert_eq!(repo_json["branches"], serde_json::Value::Array(vec![serde_json::Value::String("main".to_string())]));
    }

    // Test ID: 4
    #[test]
    fn test_init_successful() {
        let repository_path = machine_hiding::file_system_operations::get_cwd() + "/test_repo4";

        // Perform initialization
        let result = init(&repository_path);

        // Assertions
        assert_eq!(result, Ok(()));

        // Additional: Check if repository directory is created 
        let dev_path = machine_hiding::file_system_operations::join_paths(&repository_path.to_string(), &".dvcs".to_string());
        assert!(machine_hiding::file_system_operations::check_path(&dev_path));
    }

    // Test ID: 5
    #[test]
    fn test_init_duplicate_repository() {
        let repository_path = machine_hiding::file_system_operations::get_cwd();
        let first_repo = init(&repository_path);

        // Perform initialization again on same path
        let result = init(&repository_path);
        // Assertions
        assert_eq!(result, Err(DvcsError::DuplicateRepositoryError));
    }

    // Test ID: 6
    #[test]
    fn test_clone_successful() {
        // Set up mock source repo for testing. /test_repo repo has text.txt file and .dvcs/repo.json with its metadata
        let source_repository = machine_hiding::file_system_operations::get_cwd() + "/test_repo6";
        machine_hiding::file_system_operations::create_dir_all(&source_repository);
        let mut src_repo = init(&source_repository);

        let destination_repository = machine_hiding::file_system_operations::get_cwd();
        // Perform cloning
        let result = clone(&source_repository, &destination_repository);

        // Assertions
        assert_eq!(result, Ok(()));

        // Additional: Check if destination repository directory is created
        let dev_path_dest = machine_hiding::file_system_operations::join_paths(&destination_repository.to_string(), &".dvcs".to_string());
        assert!(machine_hiding::file_system_operations::check_path(&dev_path_dest));
    }

    // Test ID: 7
    #[test]
    fn test_clone_invalid_source_path() {
        let invalid_source_repository = "invalid_source_repo";
        let destination_repository = machine_hiding::file_system_operations::get_cwd();

        // Perform cloning
        let result = clone(&invalid_source_repository, &destination_repository);

        // Assertions
        assert_eq!(result, Err(DvcsError::InvalidPath));
    }

    // Test ID: 8
    #[test]
    fn test_clone_invalid_destination_path() {
        let source_repository = machine_hiding::file_system_operations::get_cwd();
        let invalid_destination_repository = "invalid_destination_repo";

        // Perform cloning
        let result = clone(&source_repository, &invalid_destination_repository);

        // Assertions
        assert_eq!(result, Err(DvcsError::InvalidPath));
    }


}