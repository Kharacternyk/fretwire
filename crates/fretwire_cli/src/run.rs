use crate::{
    Error::{self, Locale},
    Settings, exit_code, print,
};
use clap::Parser;
use fretwire_format::MovePolicy;
use std::process::ExitCode;

pub fn run() -> ExitCode {
    match Settings::try_parse().map_err(Into::into) {
        Ok(settings) => vocalize(run_silent(&settings)),
        Err(error) => vocalize(Err(error)),
    }
}

fn vocalize(result: Result<(), Error>) -> ExitCode {
    exit_code(&result.inspect_err(print))
}

fn run_silent(settings: &Settings) -> Result<(), Error<'_>> {
    fretwire::run(
        settings.path.as_deref(),
        &settings.locale.parse().map_err(|()| Locale)?,
        MovePolicy {
            marker: &settings.move_marker,
            allow_deletions: settings.allow_deletions,
            allow_external_writes: settings.allow_external_writes,
        },
        settings.one_paragraph,
        &settings.timestamp_marker,
        &settings.timestamp_pattern,
        settings.skip_disk_sync,
    )?;

    Ok(())
}
