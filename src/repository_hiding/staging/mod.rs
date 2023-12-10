/**
 * Module C.4 : Staging Module 
 * Designer : Suumil 
 * Reviewer Shunzhi
 */
    use std::collections::HashMap;

pub struct StagingModule {
    file_states: HashMap<String, FileState>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FileState {
    Added,
    Deleted,
    Updated,
}

#[derive(Debug, PartialEq)]
pub enum CommitError {
    EmptyMessage,
    // Add other error types if required
}

#[derive(Debug, PartialEq)]
pub enum TrackingError {
    FileNotFound,
    FileNotTracked,
    // Add other error types if required
}

impl StagingModule {
    // Creates a new `StagingModule` instance.
    pub fn new() -> Self {
        StagingModule {
            file_states: HashMap::new(),
        }
    }

    // Commits changes to the repository with a given message.
    pub fn commit(&self, message: &str) -> Result<(), CommitError> {
        if message.is_empty() {
            return Err(CommitError::EmptyMessage);
        }
        // TODO Implement the commit logic here
        Ok(())
    }

    // Adds or updates a file in the tracking list.
    pub fn add_or_update(&mut self, file_path: &str, state: FileState) -> Result<(), TrackingError> {
        self.file_states.insert(file_path.to_string(), state);
        Ok(())
    }

    // Marks a file as deleted in the tracking list.
    pub fn mark_deleted(&mut self, file_path: &str) -> Result<(), TrackingError> {
        if self.file_states.contains_key(file_path) {
            self.file_states.insert(file_path.to_string(), FileState::Deleted);
            Ok(())
        } else {
            Err(TrackingError::FileNotTracked)
        }
    }

    // Removes a file from the tracking list.
    pub fn remove(&mut self, file_path: &str) -> Result<(), TrackingError> {
        if self.file_states.remove(file_path).is_some() {
            Ok(())
        } else {
            Err(TrackingError::FileNotTracked)
        }
    }

    // Returns the status of tracked files.
    pub fn status(&self) -> HashMap<String, FileState> {
        self.file_states.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_success() {
        let staging_module = StagingModule::new();
        let message = "Commit message";
        let result = staging_module.commit(message);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn commit_failure() {
        let staging_module = StagingModule::new();
        let message = "";
        let result = staging_module.commit(message);
        assert_eq!(result, Err(CommitError::EmptyMessage));
    }

    #[test]
    fn add_or_update_success() {
        let mut staging_module = StagingModule::new();
        let file_path = "path/to/new_file.txt";
        let result = staging_module.add_or_update(file_path, FileState::Added);
        assert_eq!(result, Ok(()));
        let status = staging_module.status();
        assert_eq!(status.get(file_path), Some(&FileState::Added));
    }

    #[test]
    fn add_or_update_update_success() {
        let mut staging_module = StagingModule::new();
        let file_path = "path/to/existing_file.txt";
        staging_module.add_or_update(file_path, FileState::Added).unwrap();
        let result = staging_module.add_or_update(file_path, FileState::Updated);
        assert_eq!(result, Ok(()));
        let status = staging_module.status();
        assert_eq!(status.get(file_path), Some(&FileState::Updated));
    }

    #[test]
    fn mark_deleted_success() {
        let mut staging_module = StagingModule::new();
        let file_path = "path/to/existing_file.txt";
        staging_module.add_or_update(file_path, FileState::Added).unwrap();
        let result = staging_module.mark_deleted(file_path);
        assert_eq!(result, Ok(()));
        let status = staging_module.status();
        assert_eq!(status.get(file_path), Some(&FileState::Deleted));
    }

    #[test]
    fn mark_deleted_failure() {
        let mut staging_module = StagingModule::new();
        let non_existent_file = "path/to/non_existent_file.txt";
        let result = staging_module.mark_deleted(non_existent_file);
        assert_eq!(result, Err(TrackingError::FileNotTracked));
    }

    #[test]
    fn remove_success() {
        let mut staging_module = StagingModule::new();
        let file_path = "path/to/file.txt";
        staging_module.add_or_update(file_path, FileState::Added).unwrap();
        let result = staging_module.remove(file_path);
        assert_eq!(result, Ok(()));
        let status = staging_module.status();
        assert!(status.get(file_path).is_none());
    }

    #[test]
    fn remove_failure() {
        let mut staging_module = StagingModule::new();
        let non_existent_file = "path/to/non_existent_file.txt";
        let result = staging_module.remove(non_existent_file);
        assert_eq!(result, Err(TrackingError::FileNotTracked));
    }

    #[test]
    fn status_empty() {
        let staging_module = StagingModule::new();
        let status = staging_module.status();
        assert!(status.is_empty());
    }
}
