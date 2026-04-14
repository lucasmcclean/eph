use std::error::Error;
use std::fs;
use std::path::Path;

use git2::{BranchType, FetchOptions, Oid, PushOptions, RemoteCallbacks, Repository, Signature};

pub(crate) fn get_or_init_repo<P: AsRef<Path>>(
    path: P,
    repo_url: &str,
) -> Result<Repository, Box<dyn Error>> {
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
    }

    Ok(repo)
}

pub(crate) fn ensure_branch_exists(repo: &Repository, branch: &str) -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

pub(crate) fn remote_head_oid(repo: &Repository, branch: &str) -> Option<git2::Oid> {
    repo.find_reference(&format!("refs/remotes/origin/{}", branch))
        .ok()
        .and_then(|r| r.target())
}

pub(crate) fn fetch(repo: &Repository, branch: &str) -> Result<(), Box<dyn Error>> {
    let mut remote = repo.find_remote("origin")?;

    let mut fetch_options = FetchOptions::new();
    let callbacks = RemoteCallbacks::new();
    fetch_options.remote_callbacks(callbacks);

    remote.fetch(&[branch], Some(&mut fetch_options), None)?;

    Ok(())
}

pub(crate) fn commit(repo: &Repository, data_path: &Path) -> Result<(), Box<dyn Error>> {
    let filename = data_path
        .file_name()
        .and_then(|n| n.to_str())
        .expect("Data path must include a file name.");
    let contents = fs::read(data_path)?;

    let blob_oid = repo.blob(&contents)?;

    let mut tree_builder = repo.treebuilder(None)?;
    tree_builder.insert(filename, blob_oid, 0o100644)?;

    let tree_oid = tree_builder.write()?;
    let tree = repo.find_tree(tree_oid)?;

    let sig = Signature::now("eph", "local@device")?;

    let parent_commit = repo.head().ok().and_then(|h| h.peel_to_commit().ok());

    let parents: Vec<&git2::Commit> = parent_commit.as_ref().map(|c| vec![c]).unwrap_or_default();

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
) -> Result<(), Box<dyn Error>> {
    let current = remote_head_oid(repo, branch);

    if current != expected {
        return Err("remote changed since fetch".into());
    }

    let mut remote = repo.find_remote("origin")?;

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(RemoteCallbacks::new());

    remote.push(
        &[&format!("+refs/heads/{}:refs/heads/{}", branch, branch)],
        Some(&mut push_options),
    )?;

    Ok(())
}
