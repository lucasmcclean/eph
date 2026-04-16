use crate::{
    storage::DataPath,
    sync::{RepoPath, sync_with_remote},
};

pub enum SyncStatus {
    Synced,
    Failed { msg: String },
}

pub fn sync_tasks() -> SyncStatus {
    if let Err(err) = sync_with_remote(RepoPath::default(), DataPath::default()) {
        return SyncStatus::Failed {
            msg: err.to_string(),
        };
    }
    SyncStatus::Synced
}
