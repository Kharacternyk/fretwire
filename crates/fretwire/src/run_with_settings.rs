use crate::{
    Error::{self, FormatFailed, IOFailed, TimestampFailed},
    FormatInPlace, Settings,
};
use fretwire_format::{MovePolicy, Stamp, format};
use fretwire_locale::Locale;
use std::{
    borrow::Cow::{Borrowed, Owned},
    collections::HashMap,
    io::{stdin, stdout},
    iter::empty,
    path::PathBuf,
};

pub fn run_with_settings(settings: &Settings) -> Result<(), Error> {
    let stamp_value = if settings.timestamp_marker.is_empty() {
        Borrowed("")
    } else {
        Owned(
            settings
                .locale
                .timestamp(&settings.timestamp_pattern)
                .map_err(|()| TimestampFailed)?,
        )
    };
    let stamp = Stamp {
        marker: &settings.timestamp_marker,
        value: &stamp_value,
    };

    let move_policy = MovePolicy {
        marker: &settings.move_marker,
        allow_deletions: settings.allow_deletions,
        allow_external_writes: settings.allow_external_writes,
    };

    let (last_format, lines_to_move) = if let Some(path) = &settings.path {
        let (format, lines_to_move) = FormatInPlace::try_new(
            path,
            &settings.locale,
            move_policy,
            stamp,
            settings.one_paragraph,
            empty(),
            false,
        )?;

        (Some((format, Borrowed(path))), lines_to_move)
    } else {
        (
            None,
            format_stdio(&settings.locale, move_policy, stamp, settings.one_paragraph)?,
        )
    };

    let mut result: Result<(), Error> = Ok(());
    let mut formats = Vec::with_capacity(lines_to_move.len() + last_format.iter().len());

    for (path, lines) in lines_to_move {
        match FormatInPlace::try_new(
            &path,
            &settings.locale,
            MovePolicy {
                marker: "",
                allow_deletions: false,
                allow_external_writes: false,
            },
            stamp,
            settings.one_paragraph,
            lines,
            true,
        ) {
            Ok((format, lines_to_move)) => {
                assert!(lines_to_move.is_empty());

                formats.push((format, Owned(path)));
            }
            Err(error) => {
                result = Err(error);

                break;
            }
        }
    }

    if let Some(format) = last_format {
        formats.push(format);
    }

    for (format, path) in formats {
        if result.is_err() {
            let _ = format.rollback();
        } else if let Err(error) = format.commit(settings.skip_disk_sync) {
            result = Err(IOFailed {
                error: Some(error),
                path: path.into_owned(),
            });
        }
    }

    result
}

fn format_stdio(
    locale: &Locale,
    move_policy: MovePolicy,
    stamp: Stamp,
    one_paragraph: bool,
) -> Result<HashMap<PathBuf, Vec<String>>, Error> {
    format(
        &mut stdin().lock(),
        &mut stdout().lock(),
        locale,
        move_policy,
        stamp,
        one_paragraph,
        empty(),
    )
    .map_err(|error| FormatFailed { path: None, error })
    .map(|(_, lines_to_move)| lines_to_move)
}
