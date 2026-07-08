use crate::{
    Error::{self, ClapFailed, ExternalWriteForbidden, FileOpenFailed, FormatFailed},
    Settings,
};
use clap::Parser;
use fretwire_format::{Error::WriteFailed, format};
use fretwire_locale::Locale;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, ErrorKind::NotFound, stdin, stdout},
    iter,
    path::PathBuf,
    process,
};

pub fn run() -> Result<(), Error> {
    let settings = Settings::try_parse().map_err(ClapFailed)?;

    let move_lines = |lines: HashMap<String, Vec<String>>| -> Result<(), Error> {
        if !settings.allow_external_writes
            && let Some((name, strings)) = lines.iter().next()
        {
            Err(ExternalWriteForbidden {
                name: name.clone().into(),
                string: strings[0].clone(),
            })
        } else {
            for (name, lines) in lines {
                format_file(&name.into(), &settings.locale, "", lines, true, |lines| {
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
                iter::empty(),
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

        format(&mut source, &mut sink, locale, move_marker, iter::empty())
            .map_err(|error| FormatFailed { name: None, error })
    }?;

    move_lines(lines)
}

struct Sink {
    name: OsString
    file: File,
}

impl Drop for Sink {
    fn drop(&mut self) {
        let TemporaryFile(file) = self;
    }
}

fn format_file(
    name: &PathBuf,
    locale: &Locale,
    move_marker: &str,
    prepend_lines: impl IntoIterator<Item = String>,
    allow_creation: bool,
    move_lines: impl FnOnce(HashMap<String, Vec<String>>) -> Result<(), Error>,
) -> Result<(), Error> {
    let source = OpenOptions::new()
        .read(true)
        .open(name)
        .map(Some)
        .or_else(|error| match error.kind() {
            NotFound if allow_creation => Ok(None),
            _ => Err(FileOpenFailed {
                error,
                name: name.into(),
            }),
        })?;

    let mut sink_name = name.clone().into_os_string();
    sink_name.push(".");
    sink_name.push(process::id().to_string());
    sink_name.push(".fewtmp");

    let sink = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&sink_name)
        .map_err(|error| FileOpenFailed {
            error,
            name: sink_name.into(),
        })?;

    let lines = if let Some(source) = source {
        format(
            &mut BufReader::new(source),
            &mut BufWriter::new(sink),
            locale,
            move_marker,
            prepend_lines,
        )
    } else {
        format(
            &mut BufReader::new(io::empty()),
            &mut BufWriter::new(sink),
            locale,
            move_marker,
            prepend_lines,
        )
    }
    .map_err(|error| FormatFailed {
        error,
        name: Some(name.into()),
    })?;

    move_lines(lines)
}
