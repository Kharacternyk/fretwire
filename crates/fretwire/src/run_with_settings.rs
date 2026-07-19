use crate::{
    Error::{self, FormatFailed},
    FormatInPlace, IntoIOFailed, Settings,
};
use fretwire_format::{MovePolicy, format};
use fretwire_locale::Locale;
use std::{
    collections::HashMap,
    io::{stdin, stdout},
    iter::empty,
    path::PathBuf,
};

pub fn run_with_settings(settings: &Settings) -> Result<(), Error> {
    let move_policy = MovePolicy {
        marker: &settings.move_marker,
        allow_deletions: settings.allow_deletions,
        allow_external_writes: settings.allow_external_writes,
    };

    let (mut formats, lines_to_move) = if let Some(path) = &settings.path {
        let (format, lines_to_move) =
            FormatInPlace::try_new(path, &settings.locale, move_policy, empty(), false)?;

        (vec![format], lines_to_move)
    } else {
        (Vec::new(), format_stdio(&settings.locale, move_policy)?)
    };

    let mut result: Result<(), Error> = Ok(());
    let mut paths = Vec::new();

    for (path, lines) in lines_to_move {
        match FormatInPlace::try_new(&path, &settings.locale, move_policy, lines, true) {
            Ok((format, lines_to_move)) => {
                assert!(lines_to_move.is_empty());

                formats.push(format);
                paths.push(path);
            }
            Err(error) => {
                result = Err(error);

                break;
            }
        }
    }

    for (i, format) in formats.into_iter().enumerate() {
        let path = match (i, &settings.path) {
            (0, Some(path)) => path,
            (_, Some(_)) => &paths[i - 1],
            _ => &paths[i],
        };

        if result.is_err() {
            let _ = format.rollback();
        } else if let error @ Err(_) = format.commit(settings.skip_disk_sync).path(path) {
            result = error;
        }
    }

    result
}

fn format_stdio(
    locale: &Locale,
    move_policy: MovePolicy,
) -> Result<HashMap<PathBuf, Vec<String>>, Error> {
    format(
        &mut stdin().lock(),
        &mut stdout().lock(),
        locale,
        move_policy,
        empty(),
    )
    .map_err(|error| FormatFailed { path: None, error })
    .map(|(_, lines_to_move)| lines_to_move)
}
