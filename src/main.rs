use std::borrow::Cow;
use std::env;
use std::io::{BufRead, Write, stdin, stdout};
use std::process::ExitCode;

use crate::locale::Locale;
use crate::paragraph::Paragraph;

mod case;
mod locale;
mod paragraph;

fn main() -> ExitCode {
    let locale_descriptor = match env::var("FRETWIRE_LOCALE") {
        Ok(string) => Cow::Owned(string),
        Err(env::VarError::NotPresent) => Cow::Borrowed(""),
        Err(env::VarError::NotUnicode(_)) => {
            eprintln!("FRETWIRE_LOCALE is not valid UTF-8");
            return 1.into();
        }
    };
    let Some(locale) = Locale::try_new(&locale_descriptor) else {
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
