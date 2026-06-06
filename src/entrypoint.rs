use self::error::Error::{self, ClapError, FileError, ForbiddenExternalWrite, FormatError};
use crate::{
    format::{error::Error::WriteError, format},
    locale::Locale,
};
use clap::Parser;
use core::iter::empty;
use indexmap::IndexMap;
use std::{
    borrow::Cow::{self, Borrowed},
    fs::OpenOptions,
    io::{
        BufReader, Seek,
        SeekFrom::{End, Start},
        Write, stdin, stdout,
    },
};

pub mod error;

#[derive(Parser)]
#[command(version, about)]
struct Settings {
    file: Option<String>,

    #[arg(long, env = "FRETWIRE_LOCALE", default_value = "", value_parser = parse_locale)]
    locale: Locale,

    #[arg(short, long, env = "FRETWIRE_MOVE_MARKER", default_value_t = Borrowed(">>"))]
    move_marker: Cow<'static, str>,

    #[arg(short = 'w', long)]
    allow_external_writes: bool,
}

pub fn entrypoint() -> Result<(), Error> {
    let settings = Settings::try_parse().map_err(ClapError)?;
    let mut detached_rows = settings.file.as_ref().map_or_else(
        || format_stdio(&settings),
        |name| format_file(name, &settings, empty()),
    )?;

    while let Some((name, strings)) = detached_rows.pop() {
        for (name, mut strings) in format_file(&name, &settings, strings)? {
            detached_rows.entry(name).or_default().append(&mut strings);
        }
    }

    Ok(())
}

fn format_stdio(settings: &Settings) -> Result<IndexMap<String, Vec<String>>, Error> {
    let result = {
        let mut source = stdin().lock();
        let mut sink = stdout().lock();

        format(
            &mut source,
            &mut sink,
            &settings.locale,
            &settings.move_marker,
        )
    }
    .map_err(|error| FormatError { name: None, error })?;

    if !settings.allow_external_writes
        && let Some((name, strings)) = result.iter().next()
    {
        Err(ForbiddenExternalWrite {
            name: name.clone(),
            string: strings[0].clone(),
        })
    } else {
        Ok(result)
    }
}

fn format_file(
    name: &str,
    settings: &Settings,
    append_rows: impl IntoIterator<Item = String>,
) -> Result<IndexMap<String, Vec<String>>, Error> {
    let mut buffer = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(name)
        .map_err(|error| FileError {
            error,
            name: name.into(),
        })?;

    file.seek(End(0)).map_err(|error| FileError {
        error,
        name: name.into(),
    })?;

    for row in append_rows {
        file.write_all(row.as_bytes())
            .map_err(WriteError)
            .map_err(|error| FormatError {
                error,
                name: Some(name.into()),
            })?;
        file.write_all(b"\n")
            .map_err(WriteError)
            .map_err(|error| FormatError {
                error,
                name: Some(name.into()),
            })?;
    }

    file.seek(Start(0)).map_err(|error| FileError {
        error,
        name: name.into(),
    })?;

    let mut source = BufReader::new(file.try_clone().map_err(|error| FileError {
        error,
        name: name.into(),
    })?);

    let result = format(
        &mut source,
        &mut buffer,
        &settings.locale,
        &settings.move_marker,
    )
    .map_err(|error| FormatError {
        error,
        name: Some(name.into()),
    })?;

    if !settings.allow_external_writes
        && let Some((name, strings)) = result.iter().next()
    {
        Err(ForbiddenExternalWrite {
            name: name.clone(),
            string: strings[0].clone(),
        })
    } else {
        file.seek(Start(0)).map_err(|error| FileError {
            error,
            name: name.into(),
        })?;
        file.set_len(0).map_err(|error| FileError {
            error,
            name: name.into(),
        })?;
        file.write_all(&buffer)
            .map_err(WriteError)
            .map_err(|error| FormatError {
                error,
                name: Some(name.into()),
            })?;

        Ok(result)
    }
}

fn parse_locale(descriptor: &str) -> Result<Locale, &'static str> {
    descriptor.parse().map_err(|()| "Invalid locale descriptor")
}
