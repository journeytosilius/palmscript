use reqwest::blocking::Client;
use serde::Deserialize;

use crate::exchange::ExchangeEndpoints;
use crate::interval::{DeclaredMarketSource, SourceTemplate};

use super::{decode_json, QuoteFeedData};
use crate::execution::{
    ExecutionError, FeedSnapshotState, PaperExecutionSource, PriceSnapshot, TopOfBookSnapshot,
};

#[derive(Deserialize)]
struct BybitEnvelope {
    #[serde(rename = "retCode")]
    ret_code: i32,
    #[serde(rename = "retMsg")]
    ret_msg: String,
    result: BybitTickerResult,
}

#[derive(Deserialize)]
struct BybitTickerResult {
    list: Vec<BybitTicker>,
}

#[derive(Deserialize)]
struct BybitTicker {
    #[serde(rename = "bid1Price")]
    bid_1_price: String,
    #[serde(rename = "ask1Price")]
    ask_1_price: String,
    #[serde(rename = "lastPrice")]
    last_price: String,
    #[serde(rename = "markPrice")]
    mark_price: Option<String>,
}

pub(crate) fn validate(source: &DeclaredMarketSource) -> Result<(), ExecutionError> {
    if source.symbol.is_empty() {
        return Err(ExecutionError::InvalidConfig {
            message: "bybit paper execution requires a non-empty symbol".to_string(),
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
    let base = endpoints.bybit_base_url.trim_end_matches('/');
    let category = match source.template {
        SourceTemplate::BybitSpot => "spot",
        SourceTemplate::BybitUsdtPerps => "linear",
        _ => unreachable!("bybit fetch_quote_feed called for non-bybit template"),
    };
    let url = format!("{base}/v5/market/tickers");
    let payload: BybitEnvelope = decode_json(
        client
            .get(&url)
            .query(&[("category", category), ("symbol", source.symbol.as_str())])
            .send()
            .map_err(|err| ExecutionError::Fetch(err.to_string()))?,
        &url,
    )?;
    if payload.ret_code != 0 {
        return Err(ExecutionError::Fetch(format!(
            "bybit tickers returned {}: {}",
            payload.ret_code, payload.ret_msg
        )));
    }
    let ticker = payload
        .result
        .list
        .into_iter()
        .next()
        .ok_or_else(|| ExecutionError::Fetch("bybit tickers returned no rows".to_string()))?;
    let best_bid = parse_number(&ticker.bid_1_price, "bybit best bid")?;
    let best_ask = parse_number(&ticker.ask_1_price, "bybit best ask")?;
    let top_of_book = Some(TopOfBookSnapshot {
        time_ms: now_ms,
        best_bid,
        best_ask,
        mid_price: (best_bid + best_ask) / 2.0,
        state: FeedSnapshotState::Live,
    });
    let last_price = Some(PriceSnapshot {
        time_ms: now_ms,
        price: parse_number(&ticker.last_price, "bybit last price")?,
        state: FeedSnapshotState::Live,
    });
    let mark_price = ticker
        .mark_price
        .map(|price| {
            Ok(PriceSnapshot {
                time_ms: now_ms,
                price: parse_number(&price, "bybit mark price")?,
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

fn parse_number(raw: &str, field: &str) -> Result<f64, ExecutionError> {
    raw.parse::<f64>()
        .map_err(|err| ExecutionError::Fetch(format!("invalid {field} `{raw}`: {err}")))
}
