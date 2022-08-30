
use anyhow::Result;
use clap::Parser;

use cli::Opts;

use flightaware_rs as api;

mod cli;
mod version;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // By-pass everything
    if opts.version {
        println!("{}", version::version());
        return Ok(())
    }

    Ok(())
}
