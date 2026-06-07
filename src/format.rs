use self::error::Error::{self, ReadFailed, WriteFailed};
use crate::{locale::Locale, paragraph::Paragraph};
use std::{
    borrow::Cow,
    collections::HashMap,
    io::{BufRead, Write},
};

pub mod error;

pub fn format(
    source: &mut impl BufRead,
    mut sink: &mut impl Write,
    locale: &Locale,
    move_marker: &str,
) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    let mut paragraph = Paragraph::new(locale);

    for line in source.lines() {
        let mut line = line.map_err(ReadFailed)?;

        if !move_marker.is_empty()
            && let Some((body, destination)) = line.split_once(move_marker)
        {
            let destination: String = destination.trim().into();

            if !destination.is_empty() {
                line.truncate(body.len());
                result.entry(destination).or_default().push(line);
            }
        } else {
            write(&mut sink, paragraph.feed(line))?;
        }
    }

    write(&mut sink, paragraph.flush())?;

    Ok(result)
}

fn write(
    stdout: &mut impl Write,
    lines: impl IntoIterator<Item = Cow<'static, str>>,
) -> Result<(), Error> {
    for line in lines {
        for chunk in [line.as_bytes(), b"\n"] {
            stdout.write_all(chunk).map_err(WriteFailed)?;
        }
    }

    Ok(())
}
