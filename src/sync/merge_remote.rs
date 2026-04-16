use crate::{
    storage::DataPath,
    sync::{
        RepoPath,
        errors::SyncError,
        remote::{
            commit, ensure_local_branch_exists, fetch, get_or_init_local_repo, push,
            remote_head_oid,
        },
    },
};

const REPO_URL: &str = "git@github.com:lucasmcclean/tasks.git";
const BRANCH: &str = "main";

pub fn sync_with_remote(repo_path: RepoPath, data_path: DataPath) -> Result<(), SyncError> {
    let repo = get_or_init_local_repo(repo_path, REPO_URL)?;
    ensure_local_branch_exists(&repo, BRANCH)?;

    fetch(&repo, BRANCH)?;

    let lease = remote_head_oid(&repo, BRANCH);

    // TODO: Merge with local

    commit(&repo, data_path.as_ref())?;
    push(&repo, BRANCH, lease)?;

    Ok(())
}
