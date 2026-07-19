use crate::Error;
use std::{io, path::Path};

pub trait IntoIOFailed {
    type Result;

    fn path(self, path: &Path) -> Self::Result;
}

impl<T> IntoIOFailed for Option<T> {
    type Result = Result<T, Error>;

    fn path(self, path: &Path) -> Self::Result {
        self.ok_or_else(|| Error::IOFailed {
            error: None,
            path: path.into(),
        })
    }
}

impl<T> IntoIOFailed for Result<T, io::Error> {
    type Result = Result<T, Error>;

    fn path(self, path: &Path) -> Self::Result {
        self.map_err(|error| Error::IOFailed {
            error: Some(error),
            path: path.into(),
        })
    }
}
