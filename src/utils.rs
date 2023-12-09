use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

#[allow(dead_code)]
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

#[allow(dead_code)]
pub fn create_dir_all(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

#[allow(dead_code)]
pub fn write_to_file(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content)
}

#[allow(dead_code)]
pub fn read_file_to_string(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

#[allow(dead_code)]
pub fn append_to_file(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = fs::OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(content)
}

#[allow(dead_code)]
pub fn join_paths(base: &Path, relative: &Path) -> PathBuf {
    base.join(relative)
}
