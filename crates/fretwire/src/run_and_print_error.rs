use crate::{
    Error::{self, ClapFailed, FormatFailed, IOFailed},
    run,
};
use clap::error::ErrorKind::{DisplayHelp, DisplayVersion};
use fretwire_format::Error::{
    DeletionForbidden, ExternalWriteForbidden, ReadFailed, WriteFailed,
};
use std::{path::PathBuf, process::ExitCode};

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
            path,
            error: ReadFailed(error),
        } => {
            eprint!("Cannot read from ");
            eprint_path(path, "stdin");
            eprintln!(": {error}");
        }
        FormatFailed {
            path,
            error: WriteFailed(error),
        } => {
            eprint!("Cannot write to ");
            eprint_path(path, "stdout");
            eprintln!(": {error}");
        }
        FormatFailed {
            error: DeletionForbidden { line },
            ..
        } => {
            eprintln!("Deletion of {line:?} is forbidden");
        }
        FormatFailed {
            error: ExternalWriteForbidden { path, line },
            ..
        } => {
            eprintln!("External write of {line:?} to {path} is forbidden");
        }
        IOFailed { path, error } => {
            eprintln!("Cannot format {} in-place: {error}", path.display());
        }
    }
}

fn eprint_path(path: &Option<PathBuf>, default: &str) {
    match path {
        Some(path) => eprint!("{}", path.display()),
        _ => eprint!("{default}"),
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
        IOFailed { .. } => 4.into(),
        FormatFailed { .. } => 5.into(),
    }
}
