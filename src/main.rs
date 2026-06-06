use crate::{
    entrypoint::{
        entrypoint,
        error::Error::{self, ClapError, FormatError},
    },
    format::error::Error::{ReadError, WriteError},
};
use clap::error::ErrorKind::{DisplayHelp, DisplayVersion};
use std::process::ExitCode;

mod case;
mod entrypoint;
mod format;
mod locale;
mod paragraph;
mod settings;

fn main() -> ExitCode {
    entrypoint()
        .map(|()| ExitCode::SUCCESS)
        .inspect_err(print)
        .unwrap_or_else(|error| exit_code(&error))
}

fn print(error: &Error) {
    match error {
        ClapError(error) => {
            if error.print().is_err() {
                eprintln!("Error while parsing settings");
            }
        }
        FormatError(ReadError(error)) => {
            eprintln!("Error while reading from stdin: {error}");
        }
        FormatError(WriteError(error)) => {
            eprintln!("Error while writing to stdout: {error}");
        }
    }
}

fn exit_code(error: &Error) -> ExitCode {
    match error {
        ClapError(error) => match error.kind() {
            DisplayHelp | DisplayVersion => ExitCode::SUCCESS,
            _ => 1.into(),
        },
        FormatError(ReadError(_)) => 2.into(),
        FormatError(WriteError(_)) => 3.into(),
    }
}
