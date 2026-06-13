use std::{io, path::PathBuf};

pub enum Error {
    ExternalWriteForbidden {
        string: String,
        name: PathBuf,
    },
    FileOpenFailed {
        error: io::Error,
        name: PathBuf,
    },
    FormatFailed {
        error: fretwire_format::Error,
        name: Option<PathBuf>,
    },
    ClapFailed(clap::Error),
}
