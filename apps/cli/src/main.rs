mod args;
mod commands;
mod diagnostics;
mod docs;
mod format;
mod inspect;

use std::sync::Arc;

use clap::Parser;
use palmscript_logger::{error_fields, init_global, JsonLineSink, LogField, LoggerBuilder};

use crate::args::Cli;

fn main() {
    let _logger_guard =
        init_global(LoggerBuilder::new().add_sink(Arc::new(JsonLineSink::stderr())));
    let cli = Cli::parse();
    if let Err(err) = commands::run(cli) {
        error_fields(
            "cli.run",
            "PalmScript CLI command failed",
            vec![LogField::string("error", err.clone())],
        );
        eprintln!("{err}");
        std::process::exit(1);
    }
}
