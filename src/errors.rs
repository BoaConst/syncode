pub enum DvcsError {
    RepositoryAlreadyExists,
    IoError(std::io::Error),
    SerdeJsonError(serde_json::Error),
}

impl From<std::io::Error> for DvcsError {
    fn from(err: std::io::Error) -> DvcsError {
        DvcsError::IoError(err)
    }
}

impl From<serde_json::Error> for DvcsError {
    fn from(err: serde_json::Error) -> DvcsError {
        DvcsError::SerdeJsonError(err)
    }
}


impl std::fmt::Display for DvcsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DvcsError::RepositoryAlreadyExists => write!(f, "Repository already exists"),
            DvcsError::IoError(ref err) => write!(f, "IO error: {}", err),
            DvcsError::SerdeJsonError(ref err) => write!(f, "Serialization/Deserialization error: {}", err),
        }
    }
}
