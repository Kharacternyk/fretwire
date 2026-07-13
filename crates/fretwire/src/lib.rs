pub use self::{
    error::{Error, IntoIOFailed},
    formatted_file::FormattedFile,
    run::{run, run_with_settings},
    run_and_print_error::run_and_print_error,
    settings::Settings,
};

mod error;
mod formatted_file;
mod run;
mod run_and_print_error;
mod settings;
