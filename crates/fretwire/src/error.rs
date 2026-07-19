use std::{io, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    ClapFailed(clap::Error),
    LockFailed(PathBuf),
    IOFailed {
        error: Option<io::Error>,
        path: PathBuf,
    },
    FormatFailed {
        error: fretwire_format::Error,
        path: Option<PathBuf>,
    },
}
