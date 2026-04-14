use std::error::Error;
use std::fs;
use std::path::Path;

use git2::{BranchType, FetchOptions, PushOptions, RemoteCallbacks, Repository, Signature};

use crate::storage::{DataPath, RepoPath};

const REPO_URL: &str = "git@github.com:lucasmcclean/tasks.git";

fn ensure_main_branch(repo: &Repository) -> Result<(), Box<dyn std::error::Error>> {
    if repo.find_branch("main", BranchType::Local).is_ok() {
        return Ok(());
    }

    if let Ok(head) = repo.head()
        && let Ok(commit) = head.peel_to_commit()
    {
        repo.branch("main", &commit, true)?;
        return Ok(());
    }

    repo.set_head("refs/heads/main")?;

    Ok(())
}

fn get_or_init_repo<P: AsRef<Path>>(path: P) -> Result<Repository, Box<dyn Error>> {
    let path = path.as_ref();

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    if let Ok(repo) = Repository::open(path) {
        return Ok(repo);
    }

    let repo = Repository::init(path)?;
    if repo.find_remote("origin").is_err() {
        repo.remote("origin", REPO_URL)?;
    }
    Ok(repo)
}

fn set_head_to_main(repo: &Repository) -> Result<(), Box<dyn std::error::Error>> {
    repo.set_head("refs/heads/main")?;
    Ok(())
}

pub fn sync(repo_path: RepoPath, data_path: DataPath) -> Result<(), Box<dyn Error>> {
    let repo = get_or_init_repo(repo_path)?;

    ensure_main_branch(&repo)?;
    set_head_to_main(&repo)?;

    fetch(&repo)?;

    // TODO: Merge with local

    commit(&repo, data_path.as_ref())?;
    push(&repo)?;

    Ok(())
}

fn fetch(repo: &Repository) -> Result<(), Box<dyn Error>> {
    let mut remote = repo.find_remote("origin")?;

    let mut fetch_options = FetchOptions::new();
    let callbacks = RemoteCallbacks::new();
    fetch_options.remote_callbacks(callbacks);

    remote.fetch(&["main"], Some(&mut fetch_options), None)?;

    Ok(())
}

fn commit(repo: &Repository, data_path: &Path) -> Result<(), Box<dyn Error>> {
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

fn push(repo: &Repository) -> Result<(), Box<dyn Error>> {
    let mut remote = repo.find_remote("origin")?;

    let callbacks = RemoteCallbacks::new();
    // TODO: Setup SSH auth
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote.push(
        &["refs/heads/main:refs/heads/main"],
        Some(&mut push_options),
    )?;

    Ok(())
}
