/*
# Module C.1 : Initialization Module

**Designer:** Helia 
*/
// Importing modules
use std::fs;
use std::path::Path;
use crate::{machine_hiding, repository_hiding};
use uuid::Uuid;
use uuid;
use std::fmt;
use std::io;
use std::io::Write;
use std::fs::File;
use serde_json;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::io::BufReader;
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
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RepoInfo {
    root_path: String,
    tracked_files: Vec<String>,
    all_revs: Vec<RevID>,
    cur_rev: RevID,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    pub root_path: String,
    pub dev_path: String,
    repo: RepoInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RevInfo {
    rev_id: RevID,
    parent_trunk: RevID,
    parent_other: RevID,
    files: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rev {
    pub root_path: String,
    pub dev_path: String,
    pub rev_path: String,
    rev: RevInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevID {
    // #[serde(rename="UUID")]
    value: uuid::Uuid,
}
pub const EMPTY: RevID = RevID { value: Uuid::nil() };

impl fmt::Display for RepoInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root @ {}\n", self.root_path)?;
        write!(f, "Current Revision: {}\n", self.cur_rev)?;
        write!(f, "All Revisions:\n")?;
        for l in &self.all_revs {
            write!(f, "  {}\n", l.to_string())?;
        }
        write!(f, "Tracked files:\n")?;
        for l in &self.tracked_files {
            write!(f, "  {}\n", l)?;
        }
        Ok(())
    }
}
impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root Path @ {}\n", self.root_path)?;
        write!(f, ".dev Path @ {}\n", self.dev_path)?;
        write!(f, "{}\n", self.repo)?;
        Ok(())
    }
}
impl fmt::Display for RevID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.as_simple().to_string())
    }
}

impl Repo {
    pub fn clone(&self, url: &str) {
        // TO DO: implement this function
        // 1. Clone the existing repository to the new repository

    }
    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.repo).unwrap();
        machine_hiding::file_system_operations::write_string(&self.dev_path, &String::from("repo.json"), &serialized);

        // let serialized = serde_json::to_string(&self.root_path).unwrap();
        // let dev_path = machine_hiding::file_system_operations::join_paths(&self.root_path, &".dvcs".to_string());
        // machine_hiding::file_system_operations::write_string(&dev_path, &String::from("repo.json"), &serialized);
    }

    pub fn add_file(&mut self, abs_path: &String)-> Result<(), DvcsError> {

        let full_path = machine_hiding::file_system_operations::join_paths(&self.root_path, abs_path);
        assert!(machine_hiding::file_system_operations::check_path(&full_path), "File doesn't exist!");

        if !self.repo.tracked_files.contains(abs_path) {
            self.repo.tracked_files.push(abs_path.clone());
        }

        println!("Added to tracked files @ {}", abs_path);
        Ok(())
    }

    pub fn set_head_rev(&mut self, rev_id: &RevID) {
        self.repo.cur_rev = rev_id.clone();
    }

    pub fn add_rev(&mut self, rev_id: &RevID) {
        self.repo.all_revs.push(rev_id.clone());
    }

    pub fn commit(&mut self) -> repository_hiding::initialization::Rev {
        let mut rev = repository_hiding::initialization::new_rev(self, &self.repo.cur_rev, &EMPTY);

        rev.commit(&self.repo.tracked_files);
        rev.save();

        self.add_rev(rev.get_id());
        self.set_head_rev(rev.get_id());
        self.save();

        println!("Committed -> {}", rev.get_id());
        rev
    }
}

impl Rev {
    pub fn commit(&mut self, tracked_files: &Vec<String>) -> Vec<String> {
        let mut missing_files = Vec::new();
        for f_rel_path in tracked_files {
            // if machine_hiding::file_system_operations::check_path(&machine_hiding::file_system_operations::join_paths(&self.root_path, &f_rel_path)) {
            if machine_hiding::file_system_operations::check_path(f_rel_path) {
                machine_hiding::file_system_operations::my_copy_file(&self.rev_path, &self.root_path, f_rel_path);
                self.rev.files.push(f_rel_path.clone());
            } else {
                missing_files.push(f_rel_path.clone());
            }
        }
        missing_files
    }
    pub fn get_id(&self) -> &RevID {
        &self.rev.rev_id
    }
    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.rev).unwrap();
        machine_hiding::file_system_operations::write_string(&self.rev_path, &String::from("rev.json"), &serialized);
    }
}
pub fn new_revID() -> RevID {
    RevID {
        value: Uuid::new_v4()
    }
}

pub fn new_rev(repo: &Repo, trunk_id: &RevID, other_id: &RevID) -> Rev {
        let rev = RevInfo {
            rev_id: new_revID(),
            parent_trunk: trunk_id.clone(),
            parent_other: other_id.clone(),
            files: Vec::new(),
        };

        let rev_path = machine_hiding::file_system_operations::join_paths(&repo.dev_path, &rev.rev_id.to_string());
        machine_hiding::file_system_operations::create_dir_all(&rev_path);

        Rev {
            root_path: repo.root_path.clone(),
            dev_path: repo.dev_path.clone(),
            rev_path: rev_path.clone(),
            rev: rev
    }
}


pub fn init(root_path: &String) -> Result<(), DvcsError> {
    if !machine_hiding::file_system_operations::check_path(root_path) {
        machine_hiding::file_system_operations::create_dir_all(root_path);
    }

    let dev_path = machine_hiding::file_system_operations::join_paths(root_path, &".dvcs".to_string());
    assert!(!machine_hiding::file_system_operations::check_path(&dev_path), "Repo already initialized!");
    machine_hiding::file_system_operations::create_dir_all(&dev_path);

    let repo = RepoInfo {
        root_path: root_path.clone(),
        tracked_files: Vec::new(),
        all_revs: Vec::new(),
        cur_rev: EMPTY
    };

    let r = Repo {
        root_path: root_path.clone(),
        dev_path: dev_path.clone(),
        repo: repo
    };

    r.save();
    println!("Initialized repo @ {}", root_path);
    Ok(())
}

pub fn clone(root_path: &str, url: &str) -> Result<(), DvcsError> {
    todo!()
}
pub fn open(root_path: &String) -> Repo {
    let dev_path = machine_hiding::file_system_operations::join_paths(root_path, &".dvcs".to_string());
    // println!("repo at {}", dev_path);
    assert!(machine_hiding::file_system_operations::check_path(&dev_path), "Repo doesn't exist!");

    let repo_path = machine_hiding::file_system_operations::join_paths(root_path, &"repo.json".to_string());
    // println!("repo json at {}", repo_path);
    let json = machine_hiding::file_system_operations::read_line(&dev_path, &String::from("repo.json"));


    // println!("json includes: {}", json);
    // println!("ready to load");
    let repo: RepoInfo = serde_json::from_str(&json).unwrap();

    Repo {
        root_path: root_path.clone(),
        dev_path: dev_path.clone(),
        repo: repo
    }
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