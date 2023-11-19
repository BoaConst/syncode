struct RepoInfo {
    root_path: String,
    tracked_files: Vec<String>,
    all_revs: Vec<RevID>,
    cur_rev: RevID,
}

pub struct Repo {
    pub root_path: String,
    repo: RepoInfo,
}

// detail revision should be in another module but I list here for clarification
struct RevInfo {
    rev_id: RevID,
    files: Vec<String>,
}

pub struct Rev {
    pub root_path: String,
    pub rev_path: String,
    rev: RevInfo,
}


fn check_current_head(repo_path: &str) -> String{
    let target_repo = SynCode::open(repo_path).expect("non exisitent repo path");
    // this error could be handled by open function internally
    target_repo.repo.cur_rev.to_string();
    todo!() // need open function and RevID generation support function
}

fn compare_diff(file_path: &str, old_rev: &str, new_rev: &str) -> Result<String, String> {
    todo!()
    // use Crate diffy to compute the difference of new and old revision
    // if no difference, panic by reporting no different between the two revisions
}

fn inspect_file(repo_path: &str, revision: &str, file_path: &str) -> Result<String, String> {
    todo!()
    /*
    1. RevID parser
    2. open file with revid
    3. display file, error handling in ID parser and file display
    */
}

fn checkout_revision(repo_path: &str, revision_hash: &str) -> Result<(), String> {
    todo!()
    /*
    1. RevID parser
    2. open file with revid
    3. update cur_rev
    4. save settings to json
    */
}