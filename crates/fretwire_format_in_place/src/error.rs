use crate::Error::{IOFailed, LockFailed};
use std::{fs::TryLockError, io};

#[derive(Debug)]
pub enum Error {
    LockFailed,
    SizeFailed,
    IOFailed(io::Error),
    FormatFailed(fretwire_format::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        IOFailed(error)
    }
}

impl From<TryLockError> for Error {
    fn from(error: TryLockError) -> Self {
        match error {
            TryLockError::Error(error) => IOFailed(error),
            TryLockError::WouldBlock => LockFailed,
        }
    }
}
