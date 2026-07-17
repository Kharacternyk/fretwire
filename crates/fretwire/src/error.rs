use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    ClapFailed(clap::Error),
    IOFailed {
        error: io::Error,
        path: PathBuf,
    },
    FormatFailed {
        error: fretwire_format::Error,
        path: Option<PathBuf>,
    },
}
