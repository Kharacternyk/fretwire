use crate::{
    fatal_error::FatalError::{
        self, IOReadError, IOWriteError, LocaleNotUnicode, LocaleNotValid,
    },
    paragraph::Paragraph,
};
use std::{borrow::Cow, io::StdoutLock};
use std::{
    borrow::Cow::{Borrowed, Owned},
    env::{
        VarError::{NotPresent, NotUnicode},
        var,
    },
    io::{BufRead, Write, stdin, stdout},
};

pub fn format() -> Result<(), FatalError> {
    let locale_descriptor =
        var("FRETWIRE_LOCALE")
            .map(Owned)
            .or_else(|error| match error {
                NotPresent => Ok(Borrowed("")),
                NotUnicode(_) => Err(LocaleNotUnicode),
            })?;

    let locale = locale_descriptor.parse().map_err(|()| LocaleNotValid {
        descriptor: locale_descriptor,
    })?;

    let mut paragraph = Paragraph::new(locale);

    let stdin = stdin().lock();
    let mut stdout = stdout().lock();

    for line in stdin.lines() {
        let line = line.map_err(|cause| IOReadError { cause })?;

        write(&mut stdout, paragraph.feed(line))?;
    }

    write(&mut stdout, paragraph.flush())?;

    Ok(())
}

fn write(
    stdout: &mut StdoutLock,
    lines: impl IntoIterator<Item = Cow<'static, str>>,
) -> Result<(), FatalError> {
    for line in lines {
        for chunk in [line.as_bytes(), b"\n"] {
            stdout
                .write_all(chunk)
                .map_err(|cause| IOWriteError { cause })?;
        }
    }

    Ok(())
}
