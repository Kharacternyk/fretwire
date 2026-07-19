pub use self::{
    error::Error, run::run, run_and_print_error::run_and_print_error,
    run_with_settings::run_with_settings, settings::Settings,
};
use self::{format_in_place::FormatInPlace, into_io_failed::IntoIOFailed};

mod error;
mod format_in_place;
mod into_io_failed;
mod run;
mod run_and_print_error;
mod run_with_settings;
mod settings;
