use self::error::Error::{self, ReadError, WriteError};
use crate::{locale::Locale, paragraph::Paragraph};
use indexmap::IndexMap;
use std::{
    borrow::Cow,
    io::{BufRead, Write},
};

pub mod error;

pub fn format(
    source: &mut impl BufRead,
    mut sink: &mut impl Write,
    locale: &Locale,
    move_marker: &str,
) -> Result<IndexMap<String, Vec<String>>, Error> {
    let mut paragraph = Paragraph::new(locale, move_marker);

    for line in source.lines() {
        let line = line.map_err(ReadError)?;

        write(&mut sink, paragraph.feed(line))?;
    }

    write(&mut sink, paragraph.flush())?;

    Ok(paragraph.detached_rows())
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
