use crate::{
    Error::{self, ClapFailed, FormatFailed},
    IntoIOFailed, Settings,
};
use clap::Parser;
use fretwire_format::{MovePolicy, format};
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
    let move_policy = MovePolicy {
        marker: &settings.move_marker,
        allow_deletions: settings.allow_deletions,
        allow_external_writes: settings.allow_external_writes,
    };
    let move_lines = |lines: HashMap<String, Vec<String>>| -> Result<(), Error> {
        // TODO: multithreading, locking
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

    settings.file.as_ref().map_or_else(
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
    )
}

fn format_stdio(
    locale: &Locale,
    move_policy: MovePolicy,
    move_lines: impl FnOnce(HashMap<String, Vec<String>>) -> Result<(), Error>,
) -> Result<(), Error> {
    let lines = {
        let mut source = stdin().lock();
        let mut sink = stdout().lock();

        format(&mut source, &mut sink, locale, move_policy, empty())
            .map_err(|error| FormatFailed { path: None, error })
    }?;

    move_lines(lines)
}

fn format_file(
    path: &PathBuf,
    locale: &Locale,
    move_policy: MovePolicy,
    skip_disk_sync: bool,
    prepend_lines: impl IntoIterator<Item = String>,
    allow_creation: bool,
    move_lines: impl FnOnce(HashMap<String, Vec<String>>) -> Result<(), Error>,
) -> Result<(), Error> {
    const PROGRESS_MARKER: &str = "\n\nFRETWIRE IN PROGRESS\n\n";

    let mut source = OpenOptions::new()
        .read(true)
        .write(true)
        .create(allow_creation)
        .open(path)
        .path(path)?;
    let mut sink = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .path(path)?;
    let position = sink.seek(End(0)).path(path)?;

    let lines = {
        let mut buf_source = BufReader::new(&source).take(position);
        let mut buf_sink = BufWriter::new(&sink);

        buf_sink.write_all(PROGRESS_MARKER.as_bytes()).path(path)?;

        format(
            &mut buf_source,
            &mut buf_sink,
            locale,
            move_policy,
            prepend_lines,
        )
        .map_err(|error| {
            let _ = buf_sink.into_parts();
            let _ = source.set_len(position);

            FormatFailed {
                error,
                path: Some(path.into()),
            }
        })?
    };

    match move_lines(lines) {
        Ok(()) => {
            sink.seek(Start(position + (PROGRESS_MARKER.len() as u64)))
                .path(path)?;
            source.seek(Start(0)).path(path)?;

            if !skip_disk_sync {
                sink.sync_all().path(path).inspect_err(|_| {
                    let _ = source.set_len(position);
                })?;
            }

            let size = copy(&mut sink, &mut source).path(path)?;

            source.set_len(size).path(path)
        }
        error => {
            let _ = source.set_len(position);

            error
        }
    }
}
