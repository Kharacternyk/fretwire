use crate::{
    Error::{self, ClapFailed, ExternalWriteForbidden, FileOpenFailed, FormatFailed},
    run,
};
use clap::error::ErrorKind::{DisplayHelp, DisplayVersion};
use fretwire_format::Error::{ReadFailed, WriteFailed};
use std::process::ExitCode;

pub fn run_and_print_error() -> ExitCode {
    run()
        .map(|()| ExitCode::SUCCESS)
        .inspect_err(print)
        .unwrap_or_else(|error| exit_code(&error))
}

fn print(error: &Error) {
    match error {
        ClapFailed(error) => {
            if error.print().is_err() {
                eprintln!("Cannot parse settings");
            }
        }
        FormatFailed {
            name,
            error: ReadFailed(error),
        } => {
            eprint!("Cannot read from ");
            match name {
                Some(name) => eprintln!("{} {error}", name.display()),
                _ => eprintln!("stdin: {error}"),
            }
        }
        FormatFailed {
            name,
            error: WriteFailed(error),
        } => {
            eprint!("Cannot write to ");
            match name {
                Some(name) => eprintln!("{}: {error}", name.display()),
                _ => eprintln!("stdout: {error}"),
            }
        }
        FileOpenFailed { name, error } => {
            eprintln!("Cannot open {}: {error}", name.display());
        }
        ExternalWriteForbidden { string, name } => {
            eprintln!(
                "External write of {string:?} to {} is forbidden",
                name.display()
            );
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
        FileOpenFailed { .. } => 4.into(),
        ExternalWriteForbidden { .. } => 5.into(),
    }
}
