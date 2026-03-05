mod args;
mod commands;
mod data;
mod diagnostics;
mod format;

use clap::Parser;

use crate::args::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = commands::run(cli) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
