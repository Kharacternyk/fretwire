use crate::{
    fatal_error::FatalError::{
        self, IOReadError, IOWriteError, LocaleNotUnicode, LocaleNotValid,
    },
    format::format,
};
use std::process::ExitCode;

mod case;
mod fatal_error;
mod format;
mod locale;
mod paragraph;

fn main() -> ExitCode {
    format()
        .map(|()| ExitCode::SUCCESS)
        .inspect_err(print)
        .unwrap_or_else(|error| exit_code(&error).into())
}

fn print(error: &FatalError) {
    match error {
        LocaleNotUnicode => eprintln!("FRETWIRE_LOCALE is not valid UTF-8"),
        LocaleNotValid { descriptor } => {
            eprintln!("FRETWIRE_LOCALE is not a valid locale descriptor: {descriptor:?}")
        }
        IOReadError { cause } => eprintln!("Error while reading from stdin: {cause}"),
        IOWriteError { cause } => eprintln!("Error while writing to stdout: {cause}"),
    }
}

const fn exit_code(error: &FatalError) -> u8 {
    match error {
        LocaleNotUnicode | LocaleNotValid { .. } => 1,
        IOReadError { .. } => 2,
        IOWriteError { .. } => 3,
    }
}
