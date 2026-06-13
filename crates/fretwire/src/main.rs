pub use self::run_and_print_error::run_and_print_error as main;
use self::{error::Error, run::run, settings::Settings};

mod error;
mod run;
mod run_and_print_error;
mod settings;
