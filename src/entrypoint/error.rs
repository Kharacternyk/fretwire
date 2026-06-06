use crate::format;

pub enum Error {
    FormatError(format::error::Error),
    ClapError(clap::Error),
}
