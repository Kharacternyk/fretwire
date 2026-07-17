use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    IOFailed {
        error: io::Error,
        path: PathBuf,
    },
    FormatFailed {
        error: fretwire_format::Error,
        path: Option<PathBuf>,
    },
    ClapFailed(clap::Error),
}

pub trait IntoIOFailed<T> {
    type Result;

    fn path(self, path: &Path) -> Self::Result;
}

impl<T> IntoIOFailed<T> for Result<T, io::Error> {
    type Result = Result<T, Error>;

    fn path(self, path: &Path) -> Self::Result {
        self.map_err(|error| Error::IOFailed {
            error,
            path: path.into(),
        })
    }
}
