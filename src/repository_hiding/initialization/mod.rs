/*
# Module C.1 : Initialization Module

**Designer:** Helia | **Reviewer:** Shunzhi
*/

pub struct init {
    
    use crate::syncode::repository_hiding::*;      // including DvcsError, Repository, SyncodeDirectory
    
    // FOR INCREMENT 2 OR ABOVE - IGNORE FOR NOW
    // use crate::syncode::user_hiding::*;      // including DvcsAuthError, DvcsAuthConfig, DvcsAuthLog

    // implement Repository
    impl Repository {
        pub fn new(repository_name: &str) -> Repository {

            Repository {
                name: repository_name.to_string(),
                path: format!("./{}", repository_name),
            }

        }

        pub fn clone(&self, url: &str) {
            // TO DO: implement this function
            // 1. Clone the existing repository to the new repository

        }
    }

    // implement SyncodeDirectory
    impl SyncodeDirectory {
        pub fn new(repository_name: &str) -> SyncodeDirectory {

            SyncodeDirectory {
                name: format!(".syncode/{}", repository_name),
                url: format!(""),
                files: Vec::new(),
                commits: Vec::new(),
                branches: Vec::new(),
            }

            // TO DO: how to actually create the .syncode directory???

        }

        pub fn get_repository_name(url: &str) -> String {
            // TO DO: implement this function
            // 1. Get the repository name from the url
            // 2. Return the repository name
        }
    }



    pub fn init(repository_name: &str) -> Result<(), DvcsError::InitError> {
        // TO DO: implement this function

        // 1. Create a new repository
        let repository = Repository::new(repository_name);

        // 2. Create a new .syncode directory
        let syncode_directory = SyncodeDirectory::new(repository_name);

        Ok(())
    }

    pub fn clone(url: &str) -> Result<(), DvcsError::CloneError> {
        // TO DO: implement this function

        // 1. Create a new repository
        let repository_name = SyncodeDirectory::get_repository_name(url);
        let repository = Repository::new(repository_name);

        // 2. Clone the existing repository to the new repository
        repository.clone(url);

        Ok(())
    }


}