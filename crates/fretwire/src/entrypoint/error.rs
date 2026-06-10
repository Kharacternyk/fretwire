use crate::format;
use std::io;

pub enum Error {
    ExternalWriteForbidden {
        string: String,
        name: String,
    },
    FileOpenFailed {
        error: io::Error,
        name: String,
    },
    FormatFailed {
        error: format::error::Error,
        name: Option<String>,
    },
    ClapFailed(clap::Error),
}
