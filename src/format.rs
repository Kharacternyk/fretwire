use self::error::Error::{self, ReadError, WriteError};
use crate::{paragraph::Paragraph, settings::Settings};
use std::borrow::Cow;
use std::io::{BufRead, Write};

pub mod error;

pub fn format(
    source: &mut impl BufRead,
    mut sink: &mut impl Write,
    settings: &Settings,
) -> Result<(), Error> {
    let mut paragraph = Paragraph::new(settings);

    for line in source.lines() {
        let line = line.map_err(ReadError)?;

        write(&mut sink, paragraph.feed(line))?;
    }

    write(&mut sink, paragraph.flush())?;

    Ok(())
}

fn write(
    stdout: &mut impl Write,
    lines: impl IntoIterator<Item = Cow<'static, str>>,
) -> Result<(), Error> {
    for line in lines {
        for chunk in [line.as_bytes(), b"\n"] {
            stdout.write_all(chunk).map_err(WriteError)?;
        }
    }

    Ok(())
}
