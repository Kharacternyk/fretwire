use crate::format;
use std::io;

pub enum Error {
    ForbiddenExternalWrite {
        string: String,
        name: String,
    },
    FileError {
        error: io::Error,
        name: String,
    },
    FormatError {
        error: format::error::Error,
        name: Option<String>,
    },
    ClapError(clap::Error),
}
