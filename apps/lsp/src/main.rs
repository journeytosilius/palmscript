mod server;

use std::sync::Arc;

use palmscript_logger::{error_fields, init_global, JsonLineSink, LogField, LoggerBuilder};

fn main() {
    let _logger_guard =
        init_global(LoggerBuilder::new().add_sink(Arc::new(JsonLineSink::stderr())));
    if let Err(err) = server::run() {
        error_fields(
            "lsp.run",
            "PalmScript LSP exited with an error",
            vec![LogField::string("error", err.to_string())],
        );
        eprintln!("palmscript-lsp: {err}");
        std::process::exit(1);
    }
}
