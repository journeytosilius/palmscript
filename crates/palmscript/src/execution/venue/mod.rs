pub mod binance;
pub mod bybit;
pub mod gate;

use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

use crate::exchange::ExchangeEndpoints;
use crate::interval::SourceTemplate;

use super::{ExecutionError, PaperExecutionSource, PriceSnapshot, TopOfBookSnapshot};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QuoteFeedData {
    pub top_of_book: Option<TopOfBookSnapshot>,
    pub last_price: Option<PriceSnapshot>,
    pub mark_price: Option<PriceSnapshot>,
}

pub(crate) fn validate_paper_source(
    source: &crate::interval::DeclaredMarketSource,
) -> Result<(), ExecutionError> {
    match source.template {
        SourceTemplate::BinanceSpot | SourceTemplate::BinanceUsdm => binance::validate(source),
        SourceTemplate::BybitSpot | SourceTemplate::BybitUsdtPerps => bybit::validate(source),
        SourceTemplate::GateSpot | SourceTemplate::GateUsdtPerps => gate::validate(source),
    }
}

pub(crate) fn fetch_quote_feed(
    client: &Client,
    endpoints: &ExchangeEndpoints,
    source: &PaperExecutionSource,
    now_ms: i64,
) -> Result<QuoteFeedData, ExecutionError> {
    match source.template {
        SourceTemplate::BinanceSpot | SourceTemplate::BinanceUsdm => {
            binance::fetch_quote_feed(client, endpoints, source, now_ms)
        }
        SourceTemplate::BybitSpot | SourceTemplate::BybitUsdtPerps => {
            bybit::fetch_quote_feed(client, endpoints, source, now_ms)
        }
        SourceTemplate::GateSpot | SourceTemplate::GateUsdtPerps => {
            gate::fetch_quote_feed(client, endpoints, source, now_ms)
        }
    }
}

pub(crate) fn decode_json<T: DeserializeOwned>(
    response: reqwest::blocking::Response,
    path: &str,
) -> Result<T, ExecutionError> {
    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        return Err(ExecutionError::Fetch(format!(
            "HTTP {} for `{path}`: {}",
            status,
            truncate(&body)
        )));
    }
    response
        .json::<T>()
        .map_err(|err| ExecutionError::Fetch(format!("malformed `{path}` response: {err}")))
}

pub(crate) fn truncate(body: &str) -> String {
    let mut out = body.trim().replace('\n', " ");
    if out.len() > 160 {
        out.truncate(160);
        out.push_str("...");
    }
    out
}

pub(crate) fn gate_api_base(base: &str) -> String {
    if base.contains("/api/v4") {
        base.trim_end_matches('/').to_string()
    } else {
        format!("{}/api/v4", base.trim_end_matches('/'))
    }
}
