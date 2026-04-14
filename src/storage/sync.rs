use std::error::Error;

use crate::storage::{
    DataPath, RepoPath,
    git::{commit, ensure_branch_exists, fetch, get_or_init_repo, push, remote_head_oid},
};

const REPO_URL: &str = "git@github.com:lucasmcclean/tasks.git";
const BRANCH: &str = "main";

pub fn sync(repo_path: RepoPath, data_path: DataPath) -> Result<(), Box<dyn Error>> {
    let repo = get_or_init_repo(repo_path, REPO_URL)?;
    ensure_branch_exists(&repo, BRANCH)?;

    fetch(&repo, BRANCH)?;

    let lease = remote_head_oid(&repo, BRANCH);

    // TODO: Merge with local

    commit(&repo, data_path.as_ref())?;
    push(&repo, BRANCH, lease)?;

    Ok(())
}
