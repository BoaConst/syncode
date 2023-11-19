
/* FOR INCREMENT 2 ONLY - IGNORE FOR NOW */

/// This struct is used to represent the DVCS authentication configuration.
#[derive(Debug)]
pub struct DvcsAuthConfig {
    /// The name of the DVCS.
    pub name: String,
    /// The email of the DVCS.
    pub email: String,
    /// The username of the DVCS.
    pub username: String,
    /// The password of the DVCS.
    pub password: String,
}

/// This struct is used to represent the DVCS authentication log.
#[derive(Debug)]
pub struct DvcsAuthLog {
    /// The username of the DVCS.
    pub username: String,
    /// The timestamp of the DVCS.
    pub timestamp: String,
    /// The action of the DVCS.
    pub action: String,
    /// The repository of the DVCS.
    pub repository: String,
}

/// This enum type is used to represent the DVCS authentication functionalities.
/// used in the CommandParser module to parse the user input and in the interface module to execute the DVCS authentication commands.
#[derive(Debug, PartialEq)]
pub enum DvcsAuthCommand {
    check_user_authentication,
    report_user_permissions,
}

/// This error type is used to report errors related to the DVCS authentication.
#[derive(Debug)]
pub enum DvcsAuthError {
    /// This error type is used to report errors related to invalid username.
    InvalidUsername,
    /// This error type is used to report errors related to invalid password.
    InvalidPassword,
    /// This error type is used to report errors related to invalid email.
    InvalidEmail,
}
