use crate::{
    Error::{self, ClapFailed, FormatFailed},
    FormattedFile, IntoIOFailed, Settings,
};
use clap::Parser;
use fretwire_format::{MovePolicy, format};
use fretwire_locale::Locale;
use std::{
    collections::HashMap,
    io::{stdin, stdout},
    iter::empty,
};

pub fn run() -> Result<(), Error> {
    let settings = Settings::try_parse().map_err(ClapFailed)?;

    run_with_settings(&settings)
}

pub fn run_with_settings(settings: &Settings) -> Result<(), Error> {
    let move_policy = MovePolicy {
        marker: &settings.move_marker,
        allow_deletions: settings.allow_deletions,
        allow_external_writes: settings.allow_external_writes,
    };
    let move_lines = |lines: HashMap<String, Vec<String>>| -> Result<(), Error> {
        for (name, lines) in lines {
            format_file(
                &name.into(),
                &settings.locale,
                MovePolicy {
                    marker: "",
                    ..move_policy
                },
                settings.skip_disk_sync,
                lines,
                true,
                |lines| {
                    assert!(lines.is_empty());

                    Ok(())
                },
            )?;
        }

        Ok(())
    };

    if let Some(path) = settings.file {
        let formatted_file = FormattedFile::try_new(
    }

    let lines_to_move = settings.file.as_ref().map_or_else(
        || format_stdio(&settings.locale, move_policy, move_lines),
        |name| {
            format_file(
                name,
                &settings.locale,
                move_policy,
                settings.skip_disk_sync,
                empty(),
                false,
                move_lines,
            )
        },
    );
}

fn format_stdio(
    locale: &Locale,
    move_policy: MovePolicy,
) -> Result<HashMap<String, Vec<String>>, Error> {
    format(
        &mut stdin().lock(),
        &mut stdout().lock(),
        locale,
        move_policy,
        empty(),
    )
    .map_err(|error| FormatFailed { path: None, error })
}
