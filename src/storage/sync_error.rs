use std::{error, fmt, io};

#[derive(Debug)]
pub enum SyncError {
    Io(io::Error),
    Git(git2::Error),
}

impl error::Error for SyncError {}

impl From<io::Error> for SyncError {
    fn from(e: io::Error) -> Self {
        SyncError::Io(e)
    }
}

impl From<git2::Error> for SyncError {
    fn from(e: git2::Error) -> Self {
        SyncError::Git(e)
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::Io(e) => write!(f, "io error: {}", e),
            SyncError::Git(e) => write!(f, "git error: {}", e),
        }
    }
}
