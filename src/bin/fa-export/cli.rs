//! CLI handling.
//!

// External crates
//
use clap::{crate_authors, crate_name, crate_version, crate_description, Parser, AppSettings};

// Internal crates
//
use crate::version::NAME;

/// All parsable options and arguments.
#[derive(Parser, Debug)]
#[clap(name = NAME, about = crate_description!())]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::NoAutoVersion)]
pub struct Opts {
    /// debug mode
    #[clap(short = 'D', long = "debug")]
    pub debug: bool,
    /// Verbose mode
    #[clap(short = 'v', long)]
    pub verbose: bool,
    /// Display version and exit
    #[clap(short = 'V', long = "version")]
    pub version: bool,
}


