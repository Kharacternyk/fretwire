use self::{
    error::Error, run::run, run_and_print_error::run_and_print_error, settings::Settings,
};
use std::process::ExitCode;

mod error;
mod run;
mod run_and_print_error;
mod settings;

fn main() -> ExitCode {
    run_and_print_error()
}
