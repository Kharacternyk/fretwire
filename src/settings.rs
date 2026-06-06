use crate::locale::Locale;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Settings {
    #[arg(long, env = "FRETWIRE_LOCALE", default_value = "", value_parser = parse_locale)]
    pub locale: Locale,
}

fn parse_locale(descriptor: &str) -> Result<Locale, &'static str> {
    descriptor.parse().map_err(|()| "Invalid locale descriptor")
}
