use std::{
    io,
    path::{Path, PathBuf},
};

pub enum Error {
    ExternalWriteForbidden {
        string: String,
        name: PathBuf,
    },
    IOFailed {
        error: io::Error,
        name: PathBuf,
    },
    FormatFailed {
        error: fretwire_format::Error,
        name: Option<PathBuf>,
    },
    ClapFailed(clap::Error),
}

pub trait IntoIOFailed<T> {
    type Result;

    fn filename(self, name: &Path) -> Self::Result;
}

impl<T> IntoIOFailed<T> for Result<T, io::Error> {
    type Result = Result<T, Error>;

    fn filename(self, name: &Path) -> Self::Result {
        self.map_err(|error| Error::IOFailed {
            error,
            name: name.into(),
        })
    }
}
