use reqwest::blocking::Client;
use serde::Deserialize;

use crate::exchange::ExchangeEndpoints;
use crate::interval::{DeclaredMarketSource, SourceTemplate};

use super::{decode_json, gate_api_base, QuoteFeedData};
use crate::execution::{
    ExecutionError, FeedSnapshotState, PaperExecutionSource, PriceSnapshot, TopOfBookSnapshot,
};

#[derive(Deserialize)]
struct GateSpotTicker {
    highest_bid: String,
    lowest_ask: String,
    last: String,
}

#[derive(Deserialize)]
struct GateFuturesTicker {
    highest_bid: String,
    lowest_ask: String,
    last: String,
    mark_price: String,
}

pub(crate) fn validate(source: &DeclaredMarketSource) -> Result<(), ExecutionError> {
    if source.symbol.is_empty() {
        return Err(ExecutionError::InvalidConfig {
            message: "gate paper execution requires a non-empty symbol".to_string(),
        });
    }
    Ok(())
}

pub(crate) fn fetch_quote_feed(
    client: &Client,
    endpoints: &ExchangeEndpoints,
    source: &PaperExecutionSource,
    now_ms: i64,
) -> Result<QuoteFeedData, ExecutionError> {
    let base = gate_api_base(&endpoints.gate_base_url);
    match source.template {
        SourceTemplate::GateSpot => {
            let url = format!("{base}/spot/tickers");
            let rows: Vec<GateSpotTicker> = decode_json(
                client
                    .get(&url)
                    .query(&[("currency_pair", source.symbol.as_str())])
                    .send()
                    .map_err(|err| ExecutionError::Fetch(err.to_string()))?,
                &url,
            )?;
            let row = rows.into_iter().next().ok_or_else(|| {
                ExecutionError::Fetch("gate spot tickers returned no rows".to_string())
            })?;
            build_quote_feed(
                now_ms,
                &row.highest_bid,
                &row.lowest_ask,
                &row.last,
                None,
                "gate",
            )
        }
        SourceTemplate::GateUsdtPerps => {
            let url = format!("{base}/futures/usdt/tickers");
            let rows: Vec<GateFuturesTicker> = decode_json(
                client
                    .get(&url)
                    .query(&[("contract", source.symbol.as_str())])
                    .send()
                    .map_err(|err| ExecutionError::Fetch(err.to_string()))?,
                &url,
            )?;
            let row = rows.into_iter().next().ok_or_else(|| {
                ExecutionError::Fetch("gate futures tickers returned no rows".to_string())
            })?;
            build_quote_feed(
                now_ms,
                &row.highest_bid,
                &row.lowest_ask,
                &row.last,
                Some(&row.mark_price),
                "gate",
            )
        }
        _ => unreachable!("gate fetch_quote_feed called for non-gate template"),
    }
}

fn build_quote_feed(
    now_ms: i64,
    bid: &str,
    ask: &str,
    last: &str,
    mark: Option<&str>,
    label: &str,
) -> Result<QuoteFeedData, ExecutionError> {
    let best_bid = parse_number(bid, label, "best bid")?;
    let best_ask = parse_number(ask, label, "best ask")?;
    let top_of_book = Some(TopOfBookSnapshot {
        time_ms: now_ms,
        best_bid,
        best_ask,
        mid_price: (best_bid + best_ask) / 2.0,
        state: FeedSnapshotState::Live,
    });
    let last_price = Some(PriceSnapshot {
        time_ms: now_ms,
        price: parse_number(last, label, "last price")?,
        state: FeedSnapshotState::Live,
    });
    let mark_price = mark
        .map(|value| {
            Ok(PriceSnapshot {
                time_ms: now_ms,
                price: parse_number(value, label, "mark price")?,
                state: FeedSnapshotState::Live,
            })
        })
        .transpose()?;
    Ok(QuoteFeedData {
        top_of_book,
        last_price,
        mark_price,
    })
}

fn parse_number(raw: &str, venue: &str, field: &str) -> Result<f64, ExecutionError> {
    raw.parse::<f64>()
        .map_err(|err| ExecutionError::Fetch(format!("invalid {venue} {field} `{raw}`: {err}")))
}
