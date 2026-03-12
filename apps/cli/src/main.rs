mod args;
mod commands;
mod diagnostics;
mod docs;
mod format;
mod runs;

use clap::Parser;

use crate::args::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = commands::run(cli) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
