mod browser_ide;

use std::net::SocketAddr;
use std::sync::Arc;

use palmscript::ExchangeEndpoints;
use palmscript_logger::{
    error_fields, info_fields, init_global, JsonLineSink, LogField, LoggerBuilder,
};
use tokio::net::TcpListener;

use crate::browser_ide::{
    browser_ide_router, build_public_dataset_cache, public_examples, PublicIdeServerConfig,
    PublicIdeState,
};

#[tokio::main]
async fn main() {
    let _logger_guard =
        init_global(LoggerBuilder::new().add_sink(Arc::new(JsonLineSink::stderr())));
    if let Err(err) = run().await {
        error_fields(
            "ide.server.run",
            "PalmScript IDE server exited with an error",
            vec![LogField::string("error", err.clone())],
        );
        eprintln!("palmscript-ide-server: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let endpoints = ExchangeEndpoints::from_env();
    let cached = match build_public_dataset_cache(endpoints).await {
        Ok(cached) => cached,
        Err(err) => {
            error_fields(
                "ide.server.dataset_cache",
                "Public dataset cache unavailable",
                vec![LogField::string("error", err.to_string())],
            );
            eprintln!("palmscript-ide-server: dataset cache unavailable: {err}");
            Vec::new()
        }
    };
    let state = PublicIdeState::new(PublicIdeServerConfig::default(), public_examples(), cached);
    let app = browser_ide_router(state);
    let addr: SocketAddr = std::env::var("PALMSCRIPT_IDE_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
        .parse()
        .map_err(|err| format!("invalid PALMSCRIPT_IDE_BIND: {err}"))?;
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|err| format!("failed to bind {addr}: {err}"))?;
    info_fields(
        "ide.server.start",
        "PalmScript IDE server listening",
        vec![LogField::string("bind", addr.to_string())],
    );
    axum::serve(listener, app)
        .await
        .map_err(|err| format!("server error: {err}"))
}
