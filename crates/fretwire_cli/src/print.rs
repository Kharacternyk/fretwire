use crate::{
    Error::{self, Clap, Locale, Own},
    Settings,
};
use clap::CommandFactory;
use fretwire::Error::{FormatInPlaceFailed, TimestampFailed};
use fretwire_format::Error::{
    DeletionForbidden, ExternalWriteForbidden, ReadFailed, WriteFailed,
};
use fretwire_format_in_place::Error::{FormatFailed, IOFailed, LockFailed, SizeFailed};
use std::fmt::Display;

pub fn print(error: &Error) {
    let command = Settings::command();
    let style = command.get_styles().get_error();
    let print_message =
        |message: &dyn Display| anstream::eprintln!("{style}error:{style:#} {message}");

    match error {
        Clap(error) => error.print().unwrap_or_else(|error| print_message(&error)),
        Own(error) => print_own(error, print_message),
        Locale => print_message(&"invalid locale"),
    }
}

fn print_own(error: &fretwire::Error, print_message: impl Fn(&dyn Display)) {
    match error {
        FormatInPlaceFailed(LockFailed, path) => {
            print_message(&format_args!("{} is already locked", path.display()));
        }
        FormatInPlaceFailed(SizeFailed, path) => {
            print_message(&format_args!("cannot format {} in-place", path.display()));
        }
        FormatInPlaceFailed(IOFailed(error), path) => {
            print_message(&format_args!(
                "cannot format {} in-place: {error}",
                path.display()
            ));
        }
        FormatInPlaceFailed(FormatFailed(ReadFailed(error)), path) => {
            print_message(&format_args!(
                "cannot read from {}: {error}",
                path.display()
            ));
        }
        FormatInPlaceFailed(FormatFailed(WriteFailed(error)), path) => {
            print_message(&format_args!("cannot write to {}: {error}", path.display()));
        }
        fretwire::Error::FormatFailed(ReadFailed(error)) => {
            print_message(&format_args!("cannot read from stdin: {error}"));
        }
        fretwire::Error::FormatFailed(WriteFailed(error)) => {
            print_message(&format_args!("cannot write to stdout: {error}"));
        }
        FormatInPlaceFailed(FormatFailed(DeletionForbidden { line }), _)
        | fretwire::Error::FormatFailed(DeletionForbidden { line }) => {
            print_message(&format_args!("deletion of {line:?} is forbidden"));
        }
        FormatInPlaceFailed(FormatFailed(ExternalWriteForbidden { path, line }), _)
        | fretwire::Error::FormatFailed(ExternalWriteForbidden { path, line }) => {
            print_message(&format_args!(
                "external write of {line:?} to {path} is forbidden"
            ));
        }
        TimestampFailed => {
            print_message(&format_args!("timestamp creation failed"));
        }
    }
}
