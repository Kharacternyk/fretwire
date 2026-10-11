use crate::Error::FormatFailed;
use std::{borrow::Cow, path::Path};

#[derive(Debug)]
pub enum Error<'a> {
    FormatInPlaceFailed(fretwire_format_in_place::Error, Cow<'a, Path>),
    FormatFailed(fretwire_format::Error),
    TimestampFailed,
}

impl From<fretwire_format::Error> for Error<'_> {
    fn from(error: fretwire_format::Error) -> Self {
        FormatFailed(error)
    }
}
