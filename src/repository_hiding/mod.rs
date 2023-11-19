
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

    // TO DO: add more fields as needed
}

/// This SyncodeDirectory struct is used to define the .syncode directory to be created by init and clone commands.
pub struct SyncodeDirectory {

    // TO DO: add fields as needed , such as repository name, repository path, etc.
    
    // directory name
    name: String,
    // directory url
    url: String,
    // directory files
    files: Vec<String>,
    // directory commits
    commits: Vec<String>,
    // directory branches
    branches: Vec<String>,

    // TO DO: add more fields as needed
}


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
#[derive(Debug)]
pub enum DvcsError {
    /// This error type is used to report errors related to invalid command.
    InvalidCommand,
    /// This error type is used to report errors related to invalid number of arguments.
    InvalidNumberOfArguments,
    /// This error type is used to report errors related to invalid arguments.
    InvalidArguments,
    /// This error type is used to report errors related to initializing the DVCS.
    InitError,
    /// This error type is used to report errors related to adding files to the DVCS.
    AddError,
    /// This error type is used to report errors related to committing changes to the DVCS.
    CommitError,

    /// TO DO: add more error types as needed
}

