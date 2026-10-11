use crate::Error::{self, Clap, Locale, Own};
use fretwire::Error::{FormatInPlaceFailed, TimestampFailed};
use fretwire_format::Error::{
    DeletionForbidden, ExternalWriteForbidden, ReadFailed, WriteFailed,
};
use fretwire_format_in_place::Error::{FormatFailed, IOFailed, LockFailed, SizeFailed};
use std::process::ExitCode;

#[must_use]
pub fn exit_code(result: &Result<(), Error>) -> ExitCode {
    match result {
        Ok(()) => 0,
        Err(error) => match error {
            Clap(error) if error.exit_code() == 0 => 0,
            Clap(_) | Locale => 2,
            Own(error) => match error {
                FormatInPlaceFailed(FormatFailed(ReadFailed(_)), _)
                | fretwire::Error::FormatFailed(ReadFailed(_)) => 3,

                FormatInPlaceFailed(FormatFailed(WriteFailed(_)), _)
                | fretwire::Error::FormatFailed(WriteFailed(_)) => 4,

                FormatInPlaceFailed(IOFailed(_), _) => 5,

                FormatInPlaceFailed(SizeFailed, _) => 6,

                FormatInPlaceFailed(LockFailed, _) => 7,

                FormatInPlaceFailed(FormatFailed(DeletionForbidden { .. }), _)
                | fretwire::Error::FormatFailed(DeletionForbidden { .. }) => 8,

                FormatInPlaceFailed(FormatFailed(ExternalWriteForbidden { .. }), _)
                | fretwire::Error::FormatFailed(ExternalWriteForbidden { .. }) => 9,

                TimestampFailed => 10,
            },
        },
    }
    .into()
}
