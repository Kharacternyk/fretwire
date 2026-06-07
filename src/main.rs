use crate::{
    entrypoint::{
        entrypoint,
        error::Error::{
            self, ClapFailed, ExternalWriteForbidden, FileOperationFailed, FormatFailed,
        },
    },
    format::error::Error::{ReadFailed, WriteFailed},
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
        ClapFailed(error) => {
            if error.print().is_err() {
                eprintln!("Error while parsing settings");
            }
        }
        FormatFailed {
            name,
            error: ReadFailed(error),
        } => {
            eprint!("Error while reading from ");
            match name {
                Some(name) => eprintln!("{name:?}: {error}"),
                _ => eprintln!("stdin: {error}"),
            }
        }
        FormatFailed {
            name,
            error: WriteFailed(error),
        } => {
            eprint!("Error while writing to ");
            match name {
                Some(name) => eprintln!("{name:?}: {error}"),
                _ => eprintln!("stdout: {error}"),
            }
        }
        FileOperationFailed { name, error } => {
            eprintln!("Error while working with {name:?}: {error}");
        }
        ExternalWriteForbidden { string, name } => {
            eprintln!("External write of {string:?} to {name:?} is forbidden");
        }
    }
}

fn exit_code(error: &Error) -> ExitCode {
    match error {
        ClapFailed(error) => match error.kind() {
            DisplayHelp | DisplayVersion => ExitCode::SUCCESS,
            _ => 1.into(),
        },
        FormatFailed {
            error: ReadFailed(_),
            ..
        } => 2.into(),
        FormatFailed {
            error: WriteFailed(_),
            ..
        } => 3.into(),
        FileOperationFailed { .. } => 4.into(),
        ExternalWriteForbidden { .. } => 5.into(),
    }
}
