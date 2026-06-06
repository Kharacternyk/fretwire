use crate::{
    entrypoint::{
        entrypoint,
        error::Error::{self, ClapError, FileError, ForbiddenExternalWrite, FormatError},
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
        FormatError {
            name,
            error: ReadError(error),
        } => {
            eprint!("Error while reading from ");
            match name {
                Some(name) => eprint!("{name:?}: {error}"),
                _ => eprint!("stdin: {error}"),
            }
        }
        FormatError {
            name,
            error: WriteError(error),
        } => {
            eprint!("Error while writing to ");
            match name {
                Some(name) => eprint!("{name:?}: {error}"),
                _ => eprint!("stdout: {error}"),
            }
        }
        FileError { name, error } => {
            eprintln!("Error while managing {name:?} in-place: {error}");
        }
        ForbiddenExternalWrite { string, name } => {
            eprintln!("External write of {string:?} to {name:?} is forbidden");
        }
    }
}

fn exit_code(error: &Error) -> ExitCode {
    match error {
        ClapError(error) => match error.kind() {
            DisplayHelp | DisplayVersion => ExitCode::SUCCESS,
            _ => 1.into(),
        },
        FormatError {
            error: ReadError(_),
            ..
        } => 2.into(),
        FormatError {
            error: WriteError(_),
            ..
        } => 3.into(),
        FileError { .. } => 4.into(),
        ForbiddenExternalWrite { .. } => 5.into(),
    }
}
