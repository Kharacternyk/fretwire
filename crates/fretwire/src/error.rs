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
        error: fretwire_format::Error,
        name: Option<String>,
    },
    ClapFailed(clap::Error),
}
