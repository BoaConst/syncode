/*
# Module C.1 : Initialization Module

**Designer:** Helia | **Reviewer:** Shunzhi
*/

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

    // TO DO: add more fields as needed
}

// @todo: implement this as used in user_hiding::authentication_manager
pub struct RepositoryConfig {}


/// This enum type is used to represent the DVCS functionalities.
/// used in the CommandParser module to parse the user input and in the interface module to execute the DVCS commands.


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
#[derive(Serialize, Deserialize, Debug)]
struct RepoInfo {
    root_path: String,
    tracked_files: Vec<String>,
    all_revs: Vec<RevID>,
    cur_rev: RevID,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub root_path: String,
    pub dev_path: String,
    repo: RepoInfo,
}
#[derive(Serialize, Deserialize, Debug)]
struct RevInfo {
    rev_id: RevID,
    parent_trunk: RevID,
    parent_other: RevID,
    files: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Rev {
    pub root_path: String,
    pub dev_path: String,
    pub rev_path: String,
    rev: RevInfo,
}
#[derive(Serialize, Deserialize, Debug)]
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
        write!(f, ".arc Path @ {}\n", self.dev_path)?;
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

    pub fn add_file(&mut self, rel_path: &String) {

        let full_path = machine_hiding::file_system_operations::join_paths(&self.root_path, rel_path);
        assert!(machine_hiding::file_system_operations::check_path(&full_path), "File doesn't exist!");

        if !self.repo.tracked_files.contains(rel_path) {
            self.repo.tracked_files.push(rel_path.clone());
        }

        println!("Added to tracked files @ {}", rel_path);
    }

    // pub fn commit(&mut self) -> repository_hiding::initialization::Rev {
    //     let mut rev = repository_hiding::initialization::new(self, &self.repo.cur_rev, &repository_hiding::initialization::revid::EMPTY);
    //
    //     rev.commit(&self.repo.tracked_files);
    //     rev.save();
    //
    //     self.add_rev(rev.get_id());
    //     self.set_head_rev(rev.get_id());
    //     self.save();
    //
    //     println!("Committed -> {}", rev.get_id());
    //     rev
    // }
}

pub fn new_revID() -> RevID {
    RevID {
        value: Uuid::new_v4()
    }
}

// pub fn new(repo: &Repo, trunk_id: RevID, other_id: RevID) -> Rev {
//     let rev = RevInfo {
//         rev_id: new_revID(),
//         parent_trunk: trunk_id.clone(),
//         parent_other: other_id.clone(),
//         files: Vec::new(),
//     };
//
//     let rev_path = mach::join_paths(&repo.arc_path, &rev.rev_id.to_string());
//     mach::create_dir_all(&rev_path);
//
//     Rev {
//         root_path: repo.root_path.clone(),
//         arc_path: repo.arc_path.clone(),
//         rev_path: rev_path.clone(),
//         rev: rev
//     }
// }


pub fn init(root_path: &String) -> Repo {
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
    r
}

pub fn clone(root_path: &str, url: &str) -> Result<(), DvcsError> {
    todo!()
}
pub fn open(root_path: &String) -> Repo {
    let dev_path = machine_hiding::file_system_operations::join_paths(root_path, &".dvcs".to_string());
    println!("repo at {}", dev_path);
    assert!(machine_hiding::file_system_operations::check_path(&dev_path), "Repo doesn't exist!");

    let repo_path = machine_hiding::file_system_operations::join_paths(root_path, &"repo.json".to_string());
    println!("repo json at {}", repo_path);
    let json = machine_hiding::file_system_operations::read_line(&dev_path, &String::from("repo.json"));


    println!("json includes: {}", json);
    println!("ready to load");
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
    fn test_init_successful() {
        let repository_name = "test_repository".to_string();
        let result = init(&repository_name);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 2
    #[test]
    fn test_init_duplicate() {
        let repository_name = "test_repository".to_string();
        let duplicated_repository_name = "test_repository".to_string();
        let result = init(&repository_name);
        let result2 = init(&duplicated_repository_name);
        // assert_eq!(result2, Err(DvcsError::InitError::DuplicateRepositoryError));
        assert_eq!(result2, Err(DvcsError::DefaultError));
    }

    // Test ID: 3
    #[test]
    fn test_init_invalid() {
        let invalid_repository_name = "invalid_repository".to_string();
        let result = init(&invalid_repository_name);
        // assert_eq!(result, Err(DvcsError::InitError::InvalidRepositoryNameError));
        assert_eq!(result, Err(DvcsError::DefaultError));
    }

    // Test ID: 4
    #[test]
    fn test_clone_successful() {
        let url = "https://example.com/repo.dvcs".to_string();
        let repository_name = "test_repository".to_string();
        let result = clone(&repository_name, &url);
        assert_eq!(result, Ok(()));
    }

    // Test ID: 5
    #[test]
    fn test_clone_invalid_url() {
        let invalid_url = "https://invalid.invalid".to_string();
        let repository_name = "test_repository".to_string();
        let result = clone(&repository_name, &invalid_url);
        // assert_eq!(result, Err(DvcsError::CloneError::InvalidUrlError));
        assert_eq!(result, Err(DvcsError::DefaultError));
    }

    // Test ID: 6
    #[test]
    fn test_clone_network_error() {
        let network_error_url = "https://network.error".to_string();
        let repository_name = "test_repository".to_string();
        let result = clone(&repository_name, &network_error_url);
        // assert_eq!(result, Err(DvcsError::CloneError::NetworkError));
        assert_eq!(result, Err(DvcsError::DefaultError));
    }


}

