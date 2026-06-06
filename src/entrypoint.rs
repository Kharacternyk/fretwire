use self::error::Error::{self, ClapError, FormatError};
use crate::{format::format, settings::Settings};
use clap::Parser;
use std::io::{stdin, stdout};

pub mod error;

pub fn entrypoint() -> Result<(), Error> {
    let settings = Settings::try_parse().map_err(ClapError)?;
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    format(&mut stdin, &mut stdout, &settings).map_err(FormatError)?;

    Ok(())
}
