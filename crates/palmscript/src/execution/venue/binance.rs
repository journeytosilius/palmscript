use reqwest::blocking::Client;
use serde::Deserialize;

use crate::exchange::ExchangeEndpoints;
use crate::interval::{DeclaredMarketSource, SourceTemplate};

use super::{decode_json, QuoteFeedData};
use crate::execution::{
    ExecutionError, FeedSnapshotState, PaperExecutionSource, PriceSnapshot, TopOfBookSnapshot,
};

#[derive(Deserialize)]
struct BinanceBookTicker {
    #[serde(rename = "bidPrice")]
    bid_price: String,
    #[serde(rename = "askPrice")]
    ask_price: String,
}

#[derive(Deserialize)]
struct BinancePriceTicker {
    price: String,
}

#[derive(Deserialize)]
struct BinancePremiumIndex {
    #[serde(rename = "markPrice")]
    mark_price: String,
}

pub(crate) fn validate(source: &DeclaredMarketSource) -> Result<(), ExecutionError> {
    if source.symbol.is_empty() {
        return Err(ExecutionError::InvalidConfig {
            message: "binance paper execution requires a non-empty symbol".to_string(),
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
    let (base, book_path, price_path, mark_path) = match source.template {
        SourceTemplate::BinanceSpot => (
            endpoints.binance_spot_base_url.trim_end_matches('/'),
            "/api/v3/ticker/bookTicker",
            Some("/api/v3/ticker/price"),
            None,
        ),
        SourceTemplate::BinanceUsdm => (
            endpoints.binance_usdm_base_url.trim_end_matches('/'),
            "/fapi/v1/ticker/bookTicker",
            Some("/fapi/v1/ticker/price"),
            Some("/fapi/v1/premiumIndex"),
        ),
        _ => unreachable!("binance fetch_quote_feed called for non-binance template"),
    };
    let book_url = format!("{base}{book_path}");
    let book: BinanceBookTicker = decode_json(
        client
            .get(&book_url)
            .query(&[("symbol", source.symbol.as_str())])
            .send()
            .map_err(|err| ExecutionError::Fetch(err.to_string()))?,
        &book_url,
    )?;
    let best_bid = parse_number(&book.bid_price, "binance best bid")?;
    let best_ask = parse_number(&book.ask_price, "binance best ask")?;
    let mid_price = (best_bid + best_ask) / 2.0;
    let top_of_book = Some(TopOfBookSnapshot {
        time_ms: now_ms,
        best_bid,
        best_ask,
        mid_price,
        state: FeedSnapshotState::Live,
    });

    let last_price = if let Some(price_path) = price_path {
        let price_url = format!("{base}{price_path}");
        let payload: BinancePriceTicker = decode_json(
            client
                .get(&price_url)
                .query(&[("symbol", source.symbol.as_str())])
                .send()
                .map_err(|err| ExecutionError::Fetch(err.to_string()))?,
            &price_url,
        )?;
        Some(PriceSnapshot {
            time_ms: now_ms,
            price: parse_number(&payload.price, "binance last price")?,
            state: FeedSnapshotState::Live,
        })
    } else {
        None
    };

    let mark_price = if let Some(mark_path) = mark_path {
        let mark_url = format!("{base}{mark_path}");
        let payload: BinancePremiumIndex = decode_json(
            client
                .get(&mark_url)
                .query(&[("symbol", source.symbol.as_str())])
                .send()
                .map_err(|err| ExecutionError::Fetch(err.to_string()))?,
            &mark_url,
        )?;
        Some(PriceSnapshot {
            time_ms: now_ms,
            price: parse_number(&payload.mark_price, "binance mark price")?,
            state: FeedSnapshotState::Live,
        })
    } else {
        None
    };

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
