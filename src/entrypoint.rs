use self::{
    error::Error::{
        self, ClapFailed, ExternalWriteForbidden, FileOperationFailed, FormatFailed,
    },
    settings::Settings,
};
use crate::{
    format::{error::Error::WriteFailed, format},
    locale::Locale,
};
use clap::Parser;
use core::iter::empty;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{
        self, BufReader, Seek,
        SeekFrom::{End, Start},
        Write, stdin, stdout,
    },
};

pub mod error;
mod settings;

pub fn entrypoint() -> Result<(), Error> {
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
                format_file(&name, &settings.locale, "", lines, |lines| {
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

        format(&mut source, &mut sink, locale, move_marker)
            .map_err(|error| FormatFailed { name: None, error })
    }?;

    move_lines(lines)
}

fn format_file(
    name: &str,
    locale: &Locale,
    move_marker: &str,
    append_rows: impl IntoIterator<Item = String>,
    move_lines: impl FnOnce(HashMap<String, Vec<String>>) -> Result<(), Error>,
) -> Result<(), Error> {
    let file_failed = |error: io::Error| -> Error {
        FileOperationFailed {
            error,
            name: name.into(),
        }
    };
    let write_failed = |error: io::Error| -> Error {
        FormatFailed {
            error: WriteFailed(error),
            name: Some(name.into()),
        }
    };

    let mut buffer = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(name)
        .map_err(file_failed)?;

    file.seek(End(0)).map_err(file_failed)?;

    for row in append_rows {
        file.write_all(row.as_bytes()).map_err(write_failed)?;
        file.write_all(b"\n").map_err(write_failed)?;
    }

    file.seek(Start(0)).map_err(file_failed)?;

    let mut source = BufReader::new(file.try_clone().map_err(file_failed)?);

    let lines = format(&mut source, &mut buffer, locale, move_marker).map_err(|error| {
        FormatFailed {
            error,
            name: Some(name.into()),
        }
    })?;

    move_lines(lines)?;

    file.seek(Start(0)).map_err(file_failed)?;
    file.write_all(&buffer).map_err(write_failed)?;
    file.set_len(buffer.len() as u64).map_err(file_failed)?;

    Ok(())
}
