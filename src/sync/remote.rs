use std::fs;
use std::path::Path;

use git2::{
    BranchType, Cred, CredentialType, FetchOptions, Oid, PushOptions, RemoteCallbacks, Repository,
    Signature,
};

use crate::sync::errors::SyncError;

pub fn fetch(repo: &Repository, branch: &str) -> Result<(), SyncError> {
    let mut remote = repo.find_remote("origin")?;

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(add_credentials(RemoteCallbacks::new(), repo));

    remote.fetch(&[branch], Some(&mut fetch_options), None)?;

    Ok(())
}

pub(crate) fn commit(repo: &Repository, data_path: &Path) -> Result<(), SyncError> {
    let filename = data_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(SyncError::Git(git2::Error::from_str(
            "missing data file to commit",
        )))?;
    let contents = fs::read(data_path)?;

    let blob_oid = repo.blob(&contents)?;

    let mut tree_builder = repo.treebuilder(None)?;
    tree_builder.insert(filename, blob_oid, 0o100644)?;

    let tree_oid = tree_builder.write()?;
    let tree = repo.find_tree(tree_oid)?;

    let sig = Signature::now("eph", "local@device")?;

    let parent_commit = repo.head().ok().and_then(|h| h.peel_to_commit().ok());

    let parents = match &parent_commit {
        Some(c) => vec![c],
        None => vec![],
    };

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &format!("update {}", filename),
        &tree,
        &parents,
    )?;

    Ok(())
}

pub(crate) fn push(
    repo: &Repository,
    branch: &str,
    expected: Option<Oid>,
) -> Result<(), SyncError> {
    let current = remote_head_oid(repo, branch);

    if current != expected {
        return Err(SyncError::Git(git2::Error::from_str(
            "remote branch diverged",
        )));
    }

    let mut remote = repo.find_remote("origin")?;

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(add_credentials(RemoteCallbacks::new(), repo));

    remote.push(
        &[&format!("+refs/heads/{}:refs/heads/{}", branch, branch)],
        Some(&mut push_options),
    )?;

    Ok(())
}

pub(crate) fn get_or_init_local_repo<P: AsRef<Path>>(
    path: P,
    repo_url: &str,
) -> Result<Repository, SyncError> {
    let path = path.as_ref();

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(_) => Repository::init(path)?,
    };

    if repo.find_remote("origin").is_err() {
        repo.remote("origin", repo_url)?;
        repo.remote_add_fetch("origin", "+refs/heads/*:refs/remotes/origin/*")?;
    }

    Ok(repo)
}

pub(crate) fn ensure_local_branch_exists(repo: &Repository, branch: &str) -> Result<(), SyncError> {
    if repo.find_branch(branch, BranchType::Local).is_ok() {
        return Ok(());
    }

    if let Ok(head) = repo.head()
        && let Ok(commit) = head.peel_to_commit()
    {
        repo.branch(branch, &commit, true)?;
        return Ok(());
    }

    repo.set_head(&format!("refs/heads/{}", branch))?;
    repo.checkout_head(None)?;

    Ok(())
}

pub(crate) fn remote_head_oid(repo: &Repository, branch: &str) -> Option<git2::Oid> {
    repo.find_reference(&format!("refs/remotes/origin/{}", branch))
        .ok()
        .and_then(|r| r.target())
}

fn add_credentials<'a>(
    mut callbacks: RemoteCallbacks<'a>,
    repo: &'a Repository,
) -> RemoteCallbacks<'a> {
    let config = repo.config().ok();

    callbacks.credentials(move |url, username_from_url, allowed_types| {
        if let Some(config) = &config
            && let Ok(cred) = Cred::credential_helper(config, url, username_from_url)
        {
            return Ok(cred);
        }

        if allowed_types.contains(CredentialType::SSH_KEY) {
            let user = username_from_url.unwrap_or("git");
            return Cred::ssh_key_from_agent(user);
        }

        Err(git2::Error::from_str("ssh authentication failed"))
    });

    callbacks
}
