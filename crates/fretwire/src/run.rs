use crate::{
    Error::{self, ClapFailed},
    Settings, run_with_settings,
};
use clap::Parser;

pub fn run() -> Result<(), Error> {
    Settings::try_parse()
        .map_err(ClapFailed)
        .and_then(|settings| run_with_settings(&settings))
}
