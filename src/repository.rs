use std::fs;
use std::path::Path;
use serde_json;
use serde::{Serialize, Deserialize};
use crate::utils;
use crate::errors::DvcsError;

#[derive(Serialize, Deserialize)]
struct RepoState {
    tracked_files: Vec<String>,
}

pub fn init(repo_path: &Path) -> Result<(), DvcsError> {
    if !repo_path.exists() {
        fs::create_dir_all(repo_path)?;
    }

    let state_file_path = repo_path.join(".dvcs_state");
    if state_file_path.exists() {
        return Err(DvcsError::RepositoryAlreadyExists);
    }

    let initial_state = RepoState { tracked_files: Vec::new() };
    let state_contents = serde_json::to_string(&initial_state)?;
    fs::write(state_file_path, state_contents)?;

    Ok(())
}

pub fn add(repo_path: &Path, file_paths: Vec<String>) -> Result<(), DvcsError> {
    let state_file_path = repo_path.join(".dvcs_state");
    let mut state: RepoState = serde_json::from_str(&utils::read_file_to_string(&state_file_path)?)?;

    for file_path in file_paths {
        if !state.tracked_files.contains(&file_path) {
            state.tracked_files.push(file_path);
        }
    }

    let state_contents = serde_json::to_string(&state)?;
    utils::write_to_file(&state_file_path, state_contents.as_bytes())?;

    Ok(())
}

// Remove files from the tracking list
pub fn remove(repo_path: &Path, file_path: String) -> Result<(), DvcsError> {
    let state_file_path = repo_path.join(".dvcs_state");
    let mut state: RepoState = serde_json::from_str(&utils::read_file_to_string(&state_file_path)?)?;

    state.tracked_files.retain(|path| path != &file_path);

    let state_contents = serde_json::to_string(&state)?;
    utils::write_to_file(&state_file_path, state_contents.as_bytes())?;

    Ok(())
}

// Commit the current state of the repository
pub fn commit(repo_path: &Path, message: String) -> Result<(), DvcsError> {
    let log_file_path = repo_path.join(".dvcs_log");
    let commit_log = format!("Commit: {}\n", message);
    
    utils::append_to_file(&log_file_path, commit_log.as_bytes())?;

    Ok(())
}


pub fn status(repo_path: &Path) -> Result<(), DvcsError> {
    let state_file_path = repo_path.join(".dvcs_state");
    let state: RepoState = serde_json::from_str(&utils::read_file_to_string(&state_file_path)?)?;

    let mut tracked_files = state.tracked_files;
    let mut untracked_files = Vec::new();

    // List all files in repo_path and compare with tracked_files
    for entry in fs::read_dir(repo_path)? {
        let path = entry?.path();
        if path.is_file() {
            let path_str = path.to_string_lossy().into_owned();
            if !tracked_files.contains(&path_str) {
                untracked_files.push(path_str);
            } else {
                // Remove from tracked_files to find modified files later
                tracked_files.retain(|p| p != &path_str);
            }
        }
    }

    println!("Tracked files:");
    for file in tracked_files {
        println!("{}", file);
    }

    println!("\nUntracked files:");
    for file in untracked_files {
        println!("{}", file);
    }

    Ok(())
}

pub fn log(repo_path: &Path) -> Result<(), DvcsError> {
    let log_file_path = repo_path.join(".dvcs_log");

    if !log_file_path.exists() {
        println!("No commit history found.");
        return Ok(());
    }

    let log_contents = utils::read_file_to_string(&log_file_path)?;
    println!("Commit history:\n{}", log_contents);

    Ok(())
}
