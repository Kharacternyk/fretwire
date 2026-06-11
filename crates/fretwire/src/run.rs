use crate::{
    Error::{self, ClapFailed, ExternalWriteForbidden, FileOpenFailed, FormatFailed},
    Settings,
};
use atomicwrites::{
    AllowOverwrite, AtomicFile,
    Error::{Internal, User},
};
use clap::Parser;
use core::iter::empty;
use fretwire_format::{Error::WriteFailed, format};
use fretwire_locale::Locale;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufReader, BufWriter, stdin, stdout},
};

pub fn run() -> Result<(), Error> {
    let settings = Settings::try_parse().map_err(ClapFailed)?;

    let move_lines = |lines: HashMap<String, Vec<String>>| -> Result<(), Error> {
        if !settings.allow_external_writes
            && let Some((name, strings)) = lines.iter().next()
        {
            Err(ExternalWriteForbidden {
                name: name.clone(),
                string: strings[0].clone(),
            })
        } else {
            for (name, lines) in lines {
                format_file(&name, &settings.locale, "", lines, true, |lines| {
                    assert!(lines.is_empty());

                    Ok(())
                })?;
            }

            Ok(())
        }
    };

    settings.file.map_or_else(
        || format_stdio(&settings.locale, &settings.move_marker, move_lines),
        |name| {
            format_file(
                &name,
                &settings.locale,
                &settings.move_marker,
                empty(),
                false,
                move_lines,
            )
        },
    )
}

fn format_stdio(
    locale: &Locale,
    move_marker: &str,
    move_lines: impl FnOnce(HashMap<String, Vec<String>>) -> Result<(), Error>,
) -> Result<(), Error> {
    let lines = {
        let mut source = stdin().lock();
        let mut sink = stdout().lock();

        format(&mut source, &mut sink, locale, move_marker, empty())
            .map_err(|error| FormatFailed { name: None, error })
    }?;

    move_lines(lines)
}

fn format_file(
    name: &str,
    locale: &Locale,
    move_marker: &str,
    prepend_lines: impl IntoIterator<Item = String>,
    allow_creation: bool,
    move_lines: impl FnOnce(HashMap<String, Vec<String>>) -> Result<(), Error>,
) -> Result<(), Error> {
    let file = OpenOptions::new()
        .read(true)
        // Only for checking permissions
        .write(true)
        .create(allow_creation)
        .open(name)
        .map_err(|error| FileOpenFailed {
            error,
            name: name.into(),
        })?;

    let mut source = BufReader::new(file);

    AtomicFile::new(name, AllowOverwrite)
        .write(|file| {
            let mut sink = BufWriter::new(file);
            let lines = format(&mut source, &mut sink, locale, move_marker, prepend_lines)
                .map_err(|error| FormatFailed {
                error,
                name: Some(name.into()),
            })?;

            move_lines(lines)
        })
        .map_err(|error| match error {
            Internal(error) => FormatFailed {
                error: WriteFailed(error),
                name: Some(name.into()),
            },
            User(error) => error,
        })
}
