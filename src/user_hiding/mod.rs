
/* FOR INCREMENT 2 ONLY - IGNORE FOR NOW */

use crate::syncode::repository_hiding::*;

/// This struct is used to represent the DVCS authentication configuration.
#[derive(Debug)]
pub struct DvcsAuthConfig {
    /// The username of the DVCS user.
    pub username: String,
    /// The permission of the DVCS.
    pub permission: 'edit',
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

/// This error type is used to report errors related to the DVCS authentication.
#[derive(Debug)]
pub enum DvcsAuthError {
    /// This error type is used to report errors related to invalid username.
    InvalidUsername,
    /// This error type is used to report errors related to invalid permission.
    InvalidPermission,
}
