use crate::Error::{Clap, Own};

pub enum Error<'a> {
    Clap(clap::Error),
    Own(fretwire::Error<'a>),
    Locale,
}

impl From<clap::Error> for Error<'_> {
    fn from(error: clap::Error) -> Self {
        Clap(error)
    }
}

impl<'a> From<fretwire::Error<'a>> for Error<'a> {
    fn from(error: fretwire::Error<'a>) -> Self {
        Own(error)
    }
}
