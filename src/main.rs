use crate::paragraph::Paragraph;
use std::{
    borrow::Cow::{Borrowed, Owned},
    env::{
        VarError::{NotPresent, NotUnicode},
        var,
    },
    io::{BufRead, Write, stdin, stdout},
    process::ExitCode,
};

mod case;
mod locale;
mod paragraph;

fn main() -> ExitCode {
    let locale_descriptor = match var("FRETWIRE_LOCALE") {
        Ok(string) => Owned(string),
        Err(NotPresent) => Borrowed(""),
        Err(NotUnicode(_)) => {
            eprintln!("FRETWIRE_LOCALE is not valid UTF-8");
            return 1.into();
        }
    };
    let Ok(locale) = locale_descriptor.parse() else {
        eprintln!("FRETWIRE_LOCALE is not a valid locale descriptor");
        return 1.into();
    };
    let mut paragraph = Paragraph::new(locale);
    let mut stdout = stdout().lock();

    for line in stdin().lock().lines() {
        let Ok(line) = line else {
            return 2.into();
        };

        for line in paragraph.feed(line) {
            if stdout.write_all(line.as_bytes()).is_err() {
                return 3.into();
            }
            if stdout.write_all("\n".as_bytes()).is_err() {
                return 3.into();
            }
        }
    }

    for line in paragraph.flush() {
        if stdout.write_all(line.as_bytes()).is_err() {
            return 3.into();
        }
        if stdout.write_all("\n".as_bytes()).is_err() {
            return 3.into();
        }
    }

    ExitCode::SUCCESS
}
