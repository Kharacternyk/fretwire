use crate::{
    Error::{self, ClapFailed, ExternalWriteForbidden, FormatFailed},
    IntoIOFailed, Settings,
};
use clap::Parser;
use fretwire_format::format;
use fretwire_locale::Locale;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{
        BufReader, BufWriter, Read, Seek,
        SeekFrom::{End, Start},
        Write, copy, stdin, stdout,
    },
    iter::empty,
    path::PathBuf,
};

pub fn run() -> Result<(), Error> {
    let settings = Settings::try_parse().map_err(ClapFailed)?;

    run_with_settings(&settings)
}

pub fn run_with_settings(settings: &Settings) -> Result<(), Error> {
    let move_lines = |lines: HashMap<String, Vec<String>>| -> Result<(), Error> {
        if !settings.allow_external_writes
            && let Some((name, strings)) = lines.iter().next()
        {
            Err(ExternalWriteForbidden {
                name: name.clone().into(),
                string: strings[0].clone(),
            })
        } else {
            // TODO: multithreading, locking, fsync, rayon, testing
            for (name, lines) in lines {
                format_file(&name.into(), &settings.locale, "", lines, true, |lines| {
                    assert!(lines.is_empty());

                    Ok(())
                })?;
            }

            Ok(())
        }
    };

    settings.file.as_ref().map_or_else(
        || format_stdio(&settings.locale, &settings.move_marker, move_lines),
        |name| {
            format_file(
                name,
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
    name: &PathBuf,
    locale: &Locale,
    move_marker: &str,
    prepend_lines: impl IntoIterator<Item = String>,
    allow_creation: bool,
    move_lines: impl FnOnce(HashMap<String, Vec<String>>) -> Result<(), Error>,
) -> Result<(), Error> {
    const PROGRESS_MARKER: &str = "\n\nFRETWIRE IN PROGRESS\n\n";

    let mut source = OpenOptions::new()
        .read(true)
        .write(true)
        .create(allow_creation)
        .open(name)
        .filename(name)?;
    let mut sink = OpenOptions::new()
        .read(true)
        .write(true)
        .open(name)
        .filename(name)?;
    let position = sink.seek(End(0)).filename(name)?;

    let lines = {
        let mut buf_source = BufReader::new(&source).take(position);
        let mut buf_sink = BufWriter::new(&sink);

        buf_sink
            .write_all(PROGRESS_MARKER.as_bytes())
            .filename(name)?;

        format(
            &mut buf_source,
            &mut buf_sink,
            locale,
            move_marker,
            prepend_lines,
        )
        .map_err(|error| FormatFailed {
            error,
            name: Some(name.into()),
        })?
    };

    match move_lines(lines) {
        Ok(()) => {
            sink.seek(Start(position + (PROGRESS_MARKER.len() as u64)))
                .filename(name)?;
            source.seek(Start(0)).filename(name)?;

            let size = copy(&mut sink, &mut source).filename(name)?;

            source.set_len(size).filename(name)
        }
        error => {
            let _ = sink.set_len(position);
            error
        }
    }
}
