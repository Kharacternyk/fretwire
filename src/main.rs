use icu_casemap::CaseMapper;
use icu_locale::{Locale, locale};
use std::env;
use std::fmt::Display;
use std::process::ExitCode;

mod locale;
mod row;

fn main() -> ExitCode {
    let locale = match env::var("FRETWIRE_LOCALE") {
        Ok(string) => match Locale::try_from_str(&string) {
            Ok(locale) => locale,
            Err(error) => {
                return locale_error(error);
            }
        },
        Err(env::VarError::NotPresent) => locale!("und"),
        Err(env::VarError::NotUnicode(_)) => {
            return locale_error("invalid UTF-8");
        }
    };
    let case_mapper = CaseMapper::new();
    let result = case_mapper.uppercase_to_string("iii jjj", &locale.id);

    println!("{}", result);

    ExitCode::SUCCESS
}

fn locale_error<T: Display>(error: T) -> ExitCode {
    eprintln!("Error parsing FRETWIRE_LOCALE: {}", error);
    return ExitCode::from(1);
}
