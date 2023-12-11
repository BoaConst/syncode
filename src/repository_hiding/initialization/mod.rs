/*
# Module C.1 : Initialization Module

**Designer:** Helia 
*/
// Importing modules
use std::fs;
use std::path::Path;
use crate::machine_hiding::file_system_operations::join_paths;
use crate::{machine_hiding, repository_hiding};
use uuid::Uuid;
use uuid;
use std::str::FromStr;
use std::fmt;
use std::io;
use std::io::Write;
use std::fs::File;
use serde_json;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::io::BufReader;
use serde_json::json;

use super::synchronization;
/// This Repository struct is used to represent the local DVCS repository to be created by init and clone commands.
pub struct Repository {

    // repository name
    name: String,
    // repository path
    root_path: String,
    // repository files
    tracked_files: Vec<String>,
    // repository commits
    commits: Vec<String>,

    branches: Vec<String>,

    // merged from RepoInfo by Demin's
    all_revs: Vec<RevID>,
    cur_rev: RevID,

}  

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct RevInfo {
//     rev_id: RevID,
//     parent_trunk: RevID,
//     parent_other: RevID,
//     files: Vec<String>,
// }
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Rev {
//     pub root_path: String,
//     pub dev_path: String,
//     pub rev_path: String,
    
//     // rev: RevInfo,
//     rev_id: RevID,
//     parent_trunk: RevID,
//     parent_other: RevID,
//     files: Vec<String>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct RevID {
//     // #[serde(rename="UUID")]
//     value: uuid::Uuid,
// }

// pub const EMPTY: RevID = RevID { value: Uuid::nil() };

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

// // Implement Serialize for RevID
// impl Serialize for RevID {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.value.to_string().serialize(serializer)
//     }
// }
// Implement FromStr for RevID
impl std::str::FromStr for RevID {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RevID {
            value: Uuid::from_str(s)?,
        })
    }
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
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
        // write!(f, "{}", self.value.to_simple().to_string())
    }
}

impl RevID {
    pub fn is_empty(&self) -> bool {
        self.value.is_nil()
    }
}

impl Repo {
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

    // Function to copy files from the source repository to the destination repository
    fn copy_repository_contents(&self, source_path: &str) -> Result<(), DvcsError> {
        // Create a new repository object for the source repository
        let mut src_repository = Repo::new(source_path);
        src_repository.read_metadata();

        // Iterate over files in the source repository and copy them to the new repository
        for file in &src_repository.repo.tracked_files {
            let source_file_path = machine_hiding::file_system_operations::join_paths(&source_path.to_string(), file);
            
            let target_file_path = machine_hiding::file_system_operations::join_paths(&self.root_path, file);

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
        let dev_path = machine_hiding::file_system_operations::join_paths(&self.root_path, &".dvcs/repo.json".to_string());

        // Read the content of "repo.json" file
        if let Ok(metadata) = machine_hiding::file_system_operations::read_string(&dev_path) {
            // Deserialize the content into a serde_json::Value
            if let Ok(repo_json) = serde_json::from_str::<serde_json::Value>(&metadata) {
                
                // Extract the "files" array and update the Repository struct
                if let Some(files_json) = repo_json["tracked_files"].as_array() {
                    self.repo.tracked_files = files_json.iter()
                        .filter_map(|value| value.as_str())
                        .map(String::from)
                        .collect();
                }

                // // Extract the "commits" array and update the Repository struct
                // if let Some(commits_json) = repo_json["commits"].as_array() {
                //     self.commits = commits_json.iter()
                //         .filter_map(|value| value.as_str())
                //         .map(String::from)
                //         .collect();
                // }

                // // Extract the "branches" array and update the Repository struct
                // if let Some(branches_json) = repo_json["branches"].as_array() {
                //     self.branches = branches_json.iter()
                //         .filter_map(|value| value.as_str())
                //         .map(String::from)
                //         .collect();
                // }

                // Extract the "all_revs" array of RevID and update the Repository struct
                if let Some(all_revs_json) = repo_json["all_revs"].as_array() {
                    self.repo.all_revs = all_revs_json
                        .iter()
                        .filter_map(|value| RevID::from_str(value.as_str().unwrap()).ok())
                        .collect();
                }

                // Extract the "cur_rev" string and update the Repository struct
                if let Some(cur_rev_json) = repo_json["cur_rev"].as_str() {
                    self.repo.cur_rev = RevID::from_str(cur_rev_json).unwrap();
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

    pub fn remove_file(&mut self, rel_path: &String) {
        self.repo.tracked_files.retain(|x| x != rel_path);


        let full_path = machine_hiding::file_system_operations::join_paths(&self.root_path, rel_path);
        if machine_hiding::file_system_operations::check_path(&full_path) {
            machine_hiding::file_system_operations::del_file(&self.root_path, rel_path);
        }

        println!("Removed from tracked files @ {}", rel_path);
    }

    pub fn set_head_rev(&mut self, rev_id: &RevID) {
        self.repo.cur_rev = rev_id.clone();
    }

    pub fn get_head_rev_str(&self) -> String {
        self.repo.cur_rev.to_string()
    }

    pub fn add_rev(&mut self, rev_id: &RevID) {
        self.repo.all_revs.push(rev_id.clone());
    }

    pub fn commit(&mut self) -> repository_hiding::initialization::Rev {
        let mut rev = new_rev(self, &self.repo.cur_rev, &EMPTY);

        rev.commit(&self.repo.tracked_files);
        rev.save();

        self.add_rev(rev.get_id());
        self.set_head_rev(rev.get_id());
        self.save();

        println!("Committed -> {}", rev.get_id());
        rev
    }

    pub fn contains_rev(&self, rev_id: &RevID) -> bool {
        self.repo.all_revs.contains(&rev_id)
    }
    pub fn checkout(&mut self, rev_id_str: &String) -> repository_hiding::initialization::Rev {
        println!("revidstr: {}", rev_id_str);
        let rev_id = revid_parse(&rev_id_str);
        println!("revid: {}", rev_id);
        assert!(self.contains_rev(&rev_id), "Invalid revision!");

        machine_hiding::file_system_operations::del_files(&self.root_path, &self.repo.tracked_files);
        let rev = open_rev(self, &rev_id);
        rev.checkout();

        self.update_files(rev.get_files());
        self.set_head_rev(rev.get_id());
        self.save();

        println!("Checked out {}", rev_id);
        rev
    }
    pub fn update_files(&mut self, files: &Vec<String>) {
        self.repo.tracked_files.clear();
        for f in files {
            self.repo.tracked_files.push(f.to_string());
        }
    }

    pub fn new(repository_name: &str) -> Repo {

        let dev_path = machine_hiding::file_system_operations::join_paths(&repository_name.to_string(), &".dvcs".to_string());
        
        let repo = RepoInfo {
            root_path: repository_name.to_string().clone(),
            tracked_files: Vec::new(),
            all_revs: Vec::new(),
            cur_rev: EMPTY
        };
    
        let r = Repo {
            root_path: repository_name.to_string().clone(),
            dev_path: dev_path.clone(),
            repo: repo
        };

        return r;

    }

    pub fn merge(&mut self, trunk_id: &RevID, other_id: &RevID) -> Result<Rev, DvcsError> {
        if (!self.repo.all_revs.contains(trunk_id) || !self.repo.all_revs.contains(other_id)) {
            return Err(DvcsError::MergeError);
        }

        if trunk_id == other_id {
            println!("Already up to date: {}", trunk_id);
            Ok(open_rev(self, &trunk_id))
        } else if synchronization::can_reach_rev(self, &other_id, &trunk_id) {
            println!("Fast-forward -> {}", other_id);
            Ok(open_rev(self, &other_id))
        } else if synchronization::can_reach_rev(self, &trunk_id, &other_id) {
            println!("Fast-forward -> {}", trunk_id);
            Ok(open_rev(self, &trunk_id))
        } else {
            let common_id = synchronization::find_common_id(self, &trunk_id, &other_id);

            assert!(!common_id.is_empty(), "No common revision ID found!");

            let common_rev = open_rev(self, &common_id);
            let trunk_rev = open_rev(self, &trunk_id);
            let other_rev = open_rev(self, &other_id);

            let mut rev: Rev = new_rev(self, &self.repo.cur_rev, &EMPTY);

            rev.merge(&common_rev, &trunk_rev, &other_rev);
            rev.save();

            self.add_rev(rev.get_id());

            self.save();

            println!("Merged {} and  {} -> {}", trunk_id, other_id, rev.get_id());
            Ok(rev)
        }
    }


    pub fn sync(&mut self, other_repo: &Repo) {
        for other_rev_id in &other_repo.repo.all_revs {
            if !self.repo.all_revs.contains(&other_rev_id) {
                let other_rev = open_rev(&other_repo, &other_rev_id);
                let dst_path = machine_hiding::file_system_operations::join_paths(&self.dev_path, &other_rev_id.to_string());

                other_rev.copy_to(&dst_path);

                self.add_rev(&other_rev_id);
            }
        }

        self.save();
        println!("Synchronized {} with {}", self.root_path, other_repo.root_path);
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


    // pub fn checkout(&self) {
    //     for f_rel_path in &self.rev.files {
    //         assert!(machine_hiding::file_system_operations::check_path(&machine_hiding::file_system_operations::join_paths(&self.rev_path, &f_rel_path)), "File missing in a revision!");
    //         machine_hiding::file_system_operations::my_copy_file(&self.root_path, &self.rev_path, f_rel_path);
    //     }

    pub fn get_parent_trunk_id(&self) -> &RevID {
        &self.rev.parent_trunk
    }

    pub fn get_parent_other_id(&self) -> &RevID {
        &self.rev.parent_other
    }

    pub fn get_files(&self) -> &Vec<String> {
        &self.rev.files
    }


    pub fn merge(&mut self, ancestor_rev: &Rev, trunk_rev: &Rev, other_rev: &Rev) {

        let ancestor_files = ancestor_rev.get_files();
        let trunk_files = trunk_rev.get_files();
        let other_files = trunk_rev.get_files();
    
        let files = synchronization::find_all_files(ancestor_files, trunk_files, other_files);
    
        for f in &files {
            let ancestor_content = if ancestor_files.contains(&f) {Some(machine_hiding::file_system_operations::read_line(&ancestor_rev.rev_path, &f))} else { None };
            let trunk_content = if trunk_files.contains(&f) {Some(machine_hiding::file_system_operations::read_line(&trunk_rev.rev_path, &f))} else { None };
            let other_content = if other_files.contains(&f) {Some(machine_hiding::file_system_operations::read_line(&other_rev.rev_path, &f))} else { None };
            
            let merge_status = synchronization::three_way_merge(ancestor_content, trunk_content, other_content);
    
            if merge_status.is_ok() {
                let merged_content = merge_status.unwrap().clone().unwrap();
                machine_hiding::file_system_operations::write_string(&self.rev_path, &f, &merged_content);
                self.rev.files.push(f.clone());
            }
        
        }
    
    }

    pub fn copy_to(&self, dst_path: &String) {
        machine_hiding::file_system_operations::create_dir_all(dst_path);

        for f_rel_path in &self.rev.files {
            assert!(machine_hiding::file_system_operations::check_path(&machine_hiding::file_system_operations::join_paths(&self.root_path, f_rel_path)), "File missing in a revision!");
            let dest_path = machine_hiding::file_system_operations::join_paths(&dst_path, f_rel_path);
            let src_path = machine_hiding::file_system_operations::join_paths(&self.rev_path, f_rel_path);
            machine_hiding::file_system_operations::copy_file(&src_path, &dest_path);
        }

        let dest_path = machine_hiding::file_system_operations::join_paths(&dst_path, &"rev.json".to_string());
        let src_path = machine_hiding::file_system_operations::join_paths(&self.rev_path, &"rev.json".to_string());
        machine_hiding::file_system_operations::copy_file(&src_path, &dest_path);

    }

    pub fn checkout(&self) {
        for f_rel_path in &self.rev.files {
            assert!(machine_hiding::file_system_operations::check_path(&machine_hiding::file_system_operations::join_paths(&self.rev_path, &f_rel_path)), "File missing in a revision!");
            machine_hiding::file_system_operations::my_copy_file(&self.root_path, &self.rev_path, f_rel_path);
        }
    }

}
pub fn new_revID() -> RevID {
    RevID {
        value: Uuid::new_v4()
    }
}

pub fn new_revID_from_string(id: uuid::Uuid) -> RevID {
    RevID {
        value: id,
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
pub fn revid_parse(s: &String) -> RevID {
    let id = Uuid::parse_str(s).expect("Bad revision ID format!");
    RevID {
        value: id
    }
}


pub fn open_rev(repo: &Repo, rev_id: &RevID) -> Rev {
    let rev_path = join_paths(&repo.dev_path, &rev_id.to_string());
    assert!(machine_hiding::file_system_operations::check_path(&rev_path), "Repo doesn't exist!");

    let json = machine_hiding::file_system_operations::read_line(&rev_path, &String::from("rev.json"));
    let r : RevInfo = serde_json::from_str(&json).expect("Unable ot open revision, bad revision file!");

    Rev { root_path: repo.root_path.clone(), dev_path: repo.dev_path.clone(), rev_path: rev_path.clone(), rev: r }
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
    // repo_json["name"] = serde_json::Value::String(destination.to_string());
    repo_json["root_path"] = serde_json::Value::String(destination.to_string());

    let content = serde_json::to_string_pretty(&repo_json).unwrap();

    // 4. Write metadata to the destination repository
    machine_hiding::file_system_operations::write_string(&dev_path_dest, &String::from("repo.json"), &content);

    // 5. Create the Repository struct and read metadata from the destination repository
    let mut repository = Repo::new(destination);
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



// pub fn init(root_path: &String) -> Result<(), DvcsError> {
//     if !machine_hiding::file_system_operations::check_path(root_path) {
//         machine_hiding::file_system_operations::create_dir_all(root_path);
//     }

//     let dev_path = machine_hiding::file_system_operations::join_paths(root_path, &".dvcs".to_string());
//     assert!(!machine_hiding::file_system_operations::check_path(&dev_path), "Repo already initialized!");
//     machine_hiding::file_system_operations::create_dir_all(&dev_path);

//     let repo = Repo::new(&root_path);
//     repo.save();
//     println!("Initialized repo @ {}", root_path);
//     Ok(())
// }

pub fn init(repository_path: &str) -> Result<(), DvcsError> {

    // 1. Check if the repository already exists
    let dev_path = machine_hiding::file_system_operations::join_paths(&repository_path.to_string(), &".dvcs".to_string());
    if machine_hiding::file_system_operations::check_path(&dev_path) {
        return Err(DvcsError::DuplicateRepositoryError);
    }
    // 2. Create the repository directory
    machine_hiding::file_system_operations::create_dir_all(&dev_path);

    // 3. Create the Repository struct and repository files
    let mut repository = Repo::new(repository_path);
    repository.save();
    println!("Initialized repo @ {}", repository_path);
    Ok(())
}

// pub fn clone(source_path: &str, destination: &str) -> Result<(), DvcsError> {
//     // TODO 
// }
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
        let repository = Repo::new("test_repo");
        // assert_eq!(repository.name, "test_repo");
        assert_eq!(repository.root_path, "test_repo");
        // assert!(repository.files.is_empty());
        // assert!(repository.commits.is_empty());
        // assert!(repository.branches.is_empty());
    }

    // Test ID: 2
    #[test]
    fn test_repository_clone_invalid_path() {
        let repository = Repo::new("destination_repo");
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
        let mut repository = Repo::new(&cwd);

        // Mock data
        // repository.commits.push("commit1".to_string());
        // repository.branches.push("main".to_string());

        // Perform save
        repository.save();

        // Assertions (additional: check if metadata is correctly written) 
        let dev_path = machine_hiding::file_system_operations::join_paths(&cwd.to_string(), &".dvcs/repo.json".to_string());
        let metadata_content = machine_hiding::file_system_operations::read_string(&dev_path).unwrap();

        // Change the name and path to destination repository
        let mut repo_json = serde_json::from_str::<serde_json::Value>(&metadata_content).unwrap();
        // assert_eq!(repo_json["name"], serde_json::Value::String(cwd.to_string()));
        assert_eq!(repo_json["root_path"], serde_json::Value::String(cwd.to_string()));
        // assert_eq!(repo_json["commits"], serde_json::Value::Array(vec![serde_json::Value::String("commit1".to_string())]));
        // assert_eq!(repo_json["branches"], serde_json::Value::Array(vec![serde_json::Value::String("main".to_string())]));
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