// /**
//  * Module C.3 : Synchronization Module 
//  * Designer : Pranay 
//  * Reviewer Helia
//  */

use crate::{repository_hiding::initialization::*, machine_hiding};

use super::inspection;

pub fn can_reach_rev(repository: &Repo, current_id: &RevID, target_id: &RevID) -> bool {
    if current_id.is_empty() {
        false
    } else if current_id == target_id{
        true
    } else {
        let rev = open_rev(repository, current_id);
        let parent_trunk_id = rev.get_parent_trunk_id();
        let parent_other_id = rev.get_parent_other_id();

        can_reach_rev(repository, parent_trunk_id, target_id) || (!parent_other_id.is_empty() && can_reach_rev(repository, parent_other_id, target_id))
    }
}


pub fn can_reach_root(repository: &Repo, from_id: &RevID, skip_id: &RevID) -> bool {
    if from_id.is_empty() {
        true
    } else if from_id == skip_id {
        false
    } else {
        let rev = open_rev(repository, from_id);
        let parent_trunk_id = rev.get_parent_trunk_id();
        let parent_other_id = rev.get_parent_other_id();

        can_reach_root(repository, parent_trunk_id, skip_id) || (!parent_other_id.is_empty() && can_reach_root(repository, parent_other_id, skip_id))
    }
}

pub fn find_trunk_parents(repository: &Repo, from_id: &RevID) -> Vec<RevID> {
    let mut parents = Vec::new();
    let mut id = *from_id;
    while !id.is_empty() {
        if !can_reach_root(repository, from_id, &id) {
            parents.push(id.clone());
        }

        let rev = open_rev(repository, &id);
        id = *rev.get_parent_trunk_id();
    }
    parents
}

pub fn find_common_id(repository: &Repo, trunk_id: &RevID, other_id: &RevID) -> RevID {
    let parents_trunk = find_trunk_parents(repository, trunk_id);
    let parents_others = find_trunk_parents(repository, other_id);

    let mut com_id = EMPTY;

    for p in &parents_others {
        if parents_trunk.contains(p) && p != trunk_id && p != other_id {
            com_id = *p;
            break;
        }
    }
    com_id
}


pub fn find_all_files(x: &Vec<String>, y: &Vec<String>, z: &Vec<String>) -> Vec<String> {
    let mut files = Vec::new();

    for f in x {
        if !files.contains(f) {
            files.push(f.clone());
        }
    }

    for f in y {
        if !files.contains(f) {
            files.push(f.clone());
        }
    }

    for f in z {
        if !files.contains(f) {
            files.push(f.clone());
        }
    }

    files
}


// Helper function to check if two changes overlap
fn changes_overlap(change_a: &str, change_b: &str) -> bool {
    // For simplicity, assume changes have the format "Action at line X: Content"
    let line_number_a: usize = change_a.split_whitespace().nth(3).and_then(|s| s.parse().ok()).unwrap_or(0);
    let line_number_b: usize = change_b.split_whitespace().nth(3).and_then(|s| s.parse().ok()).unwrap_or(0);

    // Check if the lines are adjacent or overlapping
    line_number_a >= line_number_b && line_number_a <= line_number_b + 1
}

fn changes_are_disjoint(changes_a: &Vec<String>, changes_b: &Vec<String>) -> bool {
    // Check if there is any intersection between the sets of changes
    changes_a.iter().all(|change_a| changes_b.iter().all(|change_b| !changes_overlap(change_a, change_b)))
}



pub fn three_way_merge(ancestor: Option<String>, trunk: Option<String>, other: Option<String>) -> Result<Option<String>, DvcsError> {
    match (ancestor, trunk.clone(), other.clone()) {
        (Some(ancestor), Some(trunk), Some(other)) => {
            let changes_from_ancestor_to_trunk = inspection::diff(&ancestor, &trunk);

            let changes_from_ancestor_to_other = inspection::diff(&ancestor, &other);
            
            if changes_are_disjoint(&changes_from_ancestor_to_trunk, &changes_from_ancestor_to_other) {
                let merged_content = Some(format!("{}{}", trunk, other));
                Ok(merged_content)
            } else {
                // conflict resolution : not implemented for now
                Err(DvcsError::MergeError)
            }
        }
        _ => {
            Err(DvcsError::MergeError)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_way_merge_no_conflict() {
        let ancestor_content = Some("".to_string());
        let trunk_content = Some("Content in trunk".to_string());
        let other_content = Some("".to_string());
    
        let result = three_way_merge(ancestor_content, trunk_content, other_content);
        
        assert!(result.is_ok());
    
        if let Ok(merged_content) = result {
            if let Some(content) = merged_content {
                // Use a simpler content that avoids line-based differences
                assert_eq!(content, "Content in trunk");
            } else {
                panic!("Unexpected None value for merged content");
            }
        }
    }

    #[test]
    fn test_three_way_merge_with_conflict() {
        let ancestor_content = Some("Common ancestor content".to_string());
        let trunk_content = Some("Content in trunk".to_string());
        let other_content = Some("Modified content in other branch".to_string());

        let result = three_way_merge(ancestor_content, trunk_content, other_content);

        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err, DvcsError::MergeError);
        }
    }

    #[test]
    fn test_three_way_merge_missing_content() {
        let ancestor_content = Some("Common ancestor content".to_string());
        let trunk_content = Some("Content in trunk".to_string());
        let other_content = None;

        let result = three_way_merge(ancestor_content, trunk_content, other_content);

        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err, DvcsError::MergeError);
        }
    }
}



