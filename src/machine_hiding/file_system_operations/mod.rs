/**
 * Module A.1 : FileSystem OperationsModule
 * Designer : Suumil 
 * Reviewer Demin
 */
pub struct FileSystemOperationsModule {
    // Additional fields can be added as needed
}

#[derive(Debug, PartialEq)]
pub enum FileSystemConflict {
    LineEndings,
    PathFormat,
    CharacterEncoding,
    // Add other specific conflicts here
    Unknown, // For conflicts that cannot be categorized
}

#[derive(Debug, PartialEq)]
pub enum IoError {
    InvalidPath,
    ConflictDetectionFailed,
    OperationAdaptationFailed,
    // Other I/O errors can be added here
}

impl FileSystemOperationsModule {
    // Creates a new `FileSystemOperationsModule` instance.
    pub fn new() -> Self {
        FileSystemOperationsModule {
            // Initialize fields as needed
        }
    }

    // Identifies unique filesystem conflicts.
    pub fn identify_conflict(&self, file_path: &str) -> Result<FileSystemConflict, IoError> {
        // TODO: Implement conflict identification logic
        // This is a mock implementation. Replace with actual logic.
        if file_path.is_empty() {
            Err(IoError::InvalidPath)
        } else {
            Ok(FileSystemConflict::Unknown) // Replace with actual conflict detection
        }
    }

    // Adapts I/O operations to handle identified conflicts.
    pub fn adapt_io_operations(&self, file_path: &str, conflict: FileSystemConflict) -> Result<(), IoError> {
        // TODO: Implement I/O operation adaptation logic, replace the placeholder mock logic
        match conflict {
            FileSystemConflict::Unknown => Err(IoError::OperationAdaptationFailed),
            _ => Ok(()),
        }
    }
}


// For prototype:

use std::env;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn get_cwd() -> String {
    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    cwd
}

pub fn check_path(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn join_paths(path1: &String, path2: &String) -> String {
    let p = Path::new(path1).join(Path::new(path2));
    p.to_string_lossy().to_string()
}

pub fn create_dir_all(path: &String) {
    fs::create_dir_all(path).expect("failure to create dir");
}

pub fn write_string(path: &String, name: &String, s: &String) {
    let p = Path::new(path).join(Path::new(name));
    let mut f = File::create(p).expect("failure to create file");
    f.write_all(s.as_bytes()).expect("failure to write line");
}
pub fn is_empty_path(pbuf: &PathBuf) -> bool {
    pbuf.as_os_str() == std::ffi::OsStr::new("")
}
pub fn check_repo_dir(path: &String) -> bool {
    Path::new(path).join(Path::new(".dvcs")).exists()
}
pub fn find_repo_root_path(path: &String) -> String {
    let mut pbuf = PathBuf::from(path);
    while !is_empty_path(&pbuf) {
        let p = pbuf.to_string_lossy().to_string();
        println!("p: {}", p);
        if check_repo_dir(&p) {
            break;
        } else {
            pbuf.pop();
        }
    }
    pbuf.to_string_lossy().to_string()
}
pub fn find_rel_path(base_path: &String, full_path: &String) -> String {
    let b = full_path.starts_with(base_path);
    if b {
        full_path[base_path.len() + 1..].to_string()
    } else {
        "".to_string()
    }
}

pub fn read_line(path: &String, name: &String) -> String {
    let p = Path::new(path).join(Path::new(name));
    let mut f = File::open(p).expect("Unable to open file");
    let mut l = String::new();
    // f.read_to_string(&mut l)?.expect("Unable to read the file");
    l
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflict_identification_success() {
        let file_system_module = FileSystemOperationsModule::new();
        let file_path = "/path/to/file";
        let result = file_system_module.identify_conflict(file_path);
        assert!(result.is_ok());
    }

    #[test]
    fn conflict_identification_failure() {
        let file_system_module = FileSystemOperationsModule::new();
        let invalid_file_path = "";
        let result = file_system_module.identify_conflict(invalid_file_path);
        assert_eq!(result, Err(IoError::InvalidPath));
    }

    #[test]
    fn io_operation_adaptation_success() {
        let file_system_module = FileSystemOperationsModule::new();
        let file_path = "/path/to/file";
        let conflict = FileSystemConflict::LineEndings;
        let result = file_system_module.adapt_io_operations(file_path, conflict);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn io_operation_adaptation_failure() {
        let file_system_module = FileSystemOperationsModule::new();
        let file_path = "/path/to/file";
        let invalid_conflict = FileSystemConflict::Unknown;
        let result = file_system_module.adapt_io_operations(file_path, invalid_conflict);
        assert_eq!(result, Err(IoError::OperationAdaptationFailed));
    }
}
