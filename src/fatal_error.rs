use std::borrow::Cow;
use std::io;

pub enum FatalError {
    LocaleNotUnicode,
    LocaleNotValid { descriptor: Cow<'static, str> },
    IOReadError { cause: io::Error },
    IOWriteError { cause: io::Error },
}
