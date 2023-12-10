/**
 * Module D.2 : Inspection Module
 * Designer : Demin 
 * Reviewer Helia
 */
mod inspection_module {
    use crate::{machine_hiding, repository_hiding};
    use crate::repository_hiding::initialization;
    use crate::repository_hiding::initialization::DvcsCommand::Init;

    pub struct InspectionModule {
        
    }

    pub enum HeadInfo {
        Branch(String),
        Error(String),
    }

    impl InspectionModule {
        // Creates a new `InspectionModule` instance.
        pub fn new() -> Self {
            InspectionModule {

            }
        }

        // Checks the current head of the repository.
        pub fn check_current_head(repo_path: &str) -> HeadInfo {
            // TODO: Implement head checking logic
            if repo_path.is_empty() {
                HeadInfo::Error("Invalid repository path".to_string())
            } else {
                HeadInfo::Branch("master".to_string()) 
            }
        }

        // Compares the difference between revisions.
        pub fn compare_diff(file_path: &str, old_rev: &str, new_rev: &str) -> Result<String, String> {
            // TODO: Implement diff logic
            Ok("diff content".to_string())
        }

        // Inspects a file in a specific revision.
        pub fn inspect_file(repo_path: &str, revision: &str, file_path: &str) -> Result<String, String> {
            // TODO: Implement file inspection logic
            Ok("file content".to_string())
        }

        // Checks out a specific revision.
        pub fn checkout_revision(repo_path: &str, revision_hash: &str) -> Result<(), String> {
            // TODO: Implement checkout logic
            if revision_hash.len() != 40 { 
                Err("Invalid hash".to_string())
            } else {
                Ok(())
            }
        }


    }
}

#[cfg(test)]
mod tests {
    use super::inspection_module::*;

    #[test]
    fn head_success() {
        let result = InspectionModule::check_current_head("sample_repo");
        match result {
            HeadInfo::Branch(branch) => assert_eq!(branch, "master"),
            HeadInfo::Error(_) => panic!("Expected HeadInfo::Branch, got HeadInfo::Error."),
        }
    }

    #[test]
    fn head_failure() {
        let result = InspectionModule::check_current_head("");
        match result {
            HeadInfo::Error(error_message) => assert_eq!(error_message, "Invalid repository path"),
            _ => panic!("Expected HeadInfo::Error, got something else."),
        }
    }

    #[test]
    fn diff_success() {
        let result = InspectionModule::compare_diff("sample.txt", "HEAD^", "HEAD");
        match result {
            Ok(diff_content) => assert!(diff_content.contains("diff content")),
            Err(error) => panic!("Unexpected error: {}", error),
        }
    }

    #[test]
    fn diff_success_same() {
        let result = InspectionModule::compare_diff("sample.txt", "HEAD", "HEAD");
        match result {
            Ok(diff_content) => assert!(diff_content.contains("diff content")),
            Err(error) => panic!("Unexpected error: {}", error),
        }
    }

    #[test]
    fn inspect_existing_file() {
        let repo_path = "sample_repo";
        let result = InspectionModule::inspect_file(repo_path, "HEAD", "sample.txt");
        match result {
            Ok(content) => assert_eq!(content, "file content"),
            Err(error) => panic!("Unexpected error: {}", error),
        }
    }

    #[test]
    fn inspect_non_existing_file() {
        let repo_path = "sample_repo";
        let result = InspectionModule::inspect_file(repo_path, "HEAD", "non_existing.txt");
        match result {
            Ok(content) => panic!("Expected error, but got content: {}", content),
            Err(error) => assert!(true), // Error is expected
        }
    }

    #[test]
    fn checkout_valid_hash() {
        let repo_path = "sample_repo";
        let valid_hash = "0123456789abcdef0123456789abcdef01234567";
        let result = InspectionModule::checkout_revision(repo_path, valid_hash);
        assert!(result.is_ok());
    }

    #[test]
    fn checkout_invalid_hash() {
        let repo_path = "sample_repo";
        let invalid_hash = "0123456789";
        let result = InspectionModule::checkout_revision(repo_path, invalid_hash);
        assert_eq!(result, Err("Invalid hash".to_string()));
    }
}
