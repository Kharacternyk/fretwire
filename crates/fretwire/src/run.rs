use crate::Error::{self, FormatInPlaceFailed, TimestampFailed};
use fretwire_format::{MovePolicy, Stamp, format};
use fretwire_format_in_place::{Error::IOFailed, FormatInPlace};
use fretwire_locale::Locale;
use std::{
    borrow::Cow::{Borrowed, Owned},
    collections::HashMap,
    io::{stdin, stdout},
    iter::empty,
    path::{Path, PathBuf},
};

pub fn run<'a>(
    path: Option<&'a Path>,
    locale: &Locale,
    move_policy: MovePolicy,
    one_paragraph: bool,
    timestamp_marker: &str,
    timestamp_pattern: &str,
    skip_disk_sync: bool,
) -> Result<(), Error<'a>> {
    let stamp_value = if timestamp_marker.is_empty() {
        Borrowed("")
    } else {
        Owned(
            locale
                .timestamp(timestamp_pattern)
                .map_err(|()| TimestampFailed)?,
        )
    };
    let stamp = Stamp {
        marker: timestamp_marker,
        value: &stamp_value,
    };

    let (last_format, lines_to_move) = if let Some(path) = path {
        let (format, lines_to_move) = FormatInPlace::try_new(
            path,
            locale,
            move_policy,
            stamp,
            one_paragraph,
            empty(),
            false,
        )
        .map_err(|error| FormatInPlaceFailed(error, Borrowed(path)))?;

        (Some((format, Borrowed(path))), lines_to_move)
    } else {
        (
            None,
            format_stdio(locale, move_policy, stamp, one_paragraph)?,
        )
    };

    let mut result: Result<(), Error> = Ok(());
    let mut formats = Vec::with_capacity(lines_to_move.len() + last_format.iter().len());

    for (path, lines) in lines_to_move {
        match FormatInPlace::try_new(
            &path,
            locale,
            MovePolicy {
                marker: "",
                allow_deletions: false,
                allow_external_writes: false,
            },
            stamp,
            one_paragraph,
            lines,
            true,
        ) {
            Ok((format, lines_to_move)) => {
                assert!(lines_to_move.is_empty());

                formats.push((format, Owned(path)));
            }
            Err(error) => {
                result = Err(FormatInPlaceFailed(error, Owned(path)));

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
        } else if let Err(error) = format.commit(skip_disk_sync) {
            result = Err(FormatInPlaceFailed(IOFailed(error), path));
        }
    }

    result
}

fn format_stdio(
    locale: &Locale,
    move_policy: MovePolicy,
    stamp: Stamp,
    one_paragraph: bool,
) -> Result<HashMap<PathBuf, Vec<String>>, fretwire_format::Error> {
    format(
        &mut stdin().lock(),
        &mut stdout().lock(),
        locale,
        move_policy,
        stamp,
        one_paragraph,
        empty(),
    )
    .map(|(_, lines_to_move)| lines_to_move)
}
