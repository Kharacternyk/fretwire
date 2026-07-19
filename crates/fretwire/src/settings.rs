use clap::Parser;
use fretwire_locale::Locale;
use std::{
    borrow::Cow::{self, Borrowed},
    path::PathBuf,
};

#[derive(Parser)]
#[command(version, about)]
pub struct Settings {
    pub path: Option<PathBuf>,

    #[arg(long, env = "FRETWIRE_LOCALE", default_value = "", value_parser = parse_locale)]
    pub locale: Locale,

    #[arg(short, long, env = "FRETWIRE_MOVE_MARKER", default_value_t = Borrowed(":>"))]
    pub move_marker: Cow<'static, str>,

    #[arg(short = 'd', long)]
    pub allow_deletions: bool,

    #[arg(short = 'w', long)]
    pub allow_external_writes: bool,

    #[arg(long)]
    pub skip_disk_sync: bool,
}

fn parse_locale(descriptor: &str) -> Result<Locale, &'static str> {
    descriptor.parse().map_err(|()| "Invalid locale descriptor")
}
