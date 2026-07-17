pub use self::{
    error::Error,
    format_in_place::FormatInPlace,
    into_io_failed::IntoIOFailed,
    run::{run, run_with_settings},
    run_and_print_error::run_and_print_error,
    settings::Settings,
};

mod error;
mod format_in_place;
mod into_io_failed;
mod run;
mod run_and_print_error;
mod settings;
