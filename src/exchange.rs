//! Exchange-backed market data adapters for source-aware PalmScript runs.

use std::collections::BTreeSet;
use std::env;

use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use thiserror::Error;

use crate::compiler::CompiledProgram;
use crate::interval::{DeclaredMarketSource, Interval, SourceIntervalRef, SourceTemplate};
use crate::runtime::{Bar, SourceFeed, SourceRuntimeConfig};

const BINANCE_SPOT_URL: &str = "https://api.binance.com";
const BINANCE_USDM_URL: &str = "https://fapi.binance.com";
const HYPERLIQUID_INFO_URL: &str = "https://api.hyperliquid.xyz/info";
const BINANCE_PAGE_LIMIT: usize = 1000;
const HYPERLIQUID_PAGE_LIMIT: usize = 500;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeEndpoints {
    pub binance_spot_base_url: String,
    pub binance_usdm_base_url: String,
    pub hyperliquid_info_url: String,
}

impl Default for ExchangeEndpoints {
    fn default() -> Self {
        Self {
            binance_spot_base_url: BINANCE_SPOT_URL.to_string(),
            binance_usdm_base_url: BINANCE_USDM_URL.to_string(),
            hyperliquid_info_url: HYPERLIQUID_INFO_URL.to_string(),
        }
    }
}

impl ExchangeEndpoints {
    pub fn from_env() -> Self {
        Self {
            binance_spot_base_url: env::var("PALMSCRIPT_BINANCE_SPOT_BASE_URL")
                .unwrap_or_else(|_| BINANCE_SPOT_URL.to_string()),
            binance_usdm_base_url: env::var("PALMSCRIPT_BINANCE_USDM_BASE_URL")
                .unwrap_or_else(|_| BINANCE_USDM_URL.to_string()),
            hyperliquid_info_url: env::var("PALMSCRIPT_HYPERLIQUID_INFO_URL")
                .unwrap_or_else(|_| HYPERLIQUID_INFO_URL.to_string()),
        }
    }
}

#[derive(Debug, Error)]
pub enum ExchangeFetchError {
    #[error("exchange-backed runs require a base interval declaration")]
    MissingBaseInterval,
    #[error("exchange-backed runs require at least one `source` declaration")]
    MissingSources,
    #[error("invalid market time window: from {from_ms} must be less than to {to_ms}")]
    InvalidTimeWindow { from_ms: i64, to_ms: i64 },
    #[error("source `{alias}` with template `{template}` does not support interval `{interval}`")]
    UnsupportedInterval {
        alias: String,
        template: &'static str,
        interval: &'static str,
    },
    #[error("failed to fetch `{alias}` ({template}) `{symbol}` {interval}: {message}")]
    RequestFailed {
        alias: String,
        template: &'static str,
        symbol: String,
        interval: &'static str,
        message: String,
    },
    #[error("malformed response for `{alias}` ({template}) `{symbol}` {interval}: {message}")]
    MalformedResponse {
        alias: String,
        template: &'static str,
        symbol: String,
        interval: &'static str,
        message: String,
    },
    #[error("no data returned for `{alias}` ({template}) `{symbol}` {interval}")]
    NoData {
        alias: String,
        template: &'static str,
        symbol: String,
        interval: &'static str,
    },
    #[error("unknown Hyperliquid spot symbol `{symbol}`")]
    UnknownHyperliquidSpotSymbol { symbol: String },
}

pub fn fetch_source_runtime_config(
    compiled: &CompiledProgram,
    from_ms: i64,
    to_ms: i64,
    endpoints: &ExchangeEndpoints,
) -> Result<SourceRuntimeConfig, ExchangeFetchError> {
    if from_ms >= to_ms {
        return Err(ExchangeFetchError::InvalidTimeWindow { from_ms, to_ms });
    }
    let base_interval = compiled
        .program
        .base_interval
        .ok_or(ExchangeFetchError::MissingBaseInterval)?;
    if compiled.program.declared_sources.is_empty() {
        return Err(ExchangeFetchError::MissingSources);
    }

    let client = Client::builder()
        .build()
        .map_err(|err| ExchangeFetchError::RequestFailed {
            alias: "client".to_string(),
            template: "http",
            symbol: String::new(),
            interval: "",
            message: err.to_string(),
        })?;

    let mut required = BTreeSet::<SourceIntervalRef>::new();
    for source in &compiled.program.declared_sources {
        required.insert(SourceIntervalRef {
            source_id: source.id,
            interval: base_interval,
        });
    }
    required.extend(compiled.program.source_intervals.iter().copied());

    let mut feeds = Vec::new();
    for requirement in required {
        let source = compiled
            .program
            .declared_sources
            .iter()
            .find(|source| source.id == requirement.source_id)
            .expect("compiled source interval references should resolve");
        if !source.template.supports_interval(requirement.interval) {
            return Err(ExchangeFetchError::UnsupportedInterval {
                alias: source.alias.clone(),
                template: source.template.as_str(),
                interval: requirement.interval.as_str(),
            });
        }
        let bars = fetch_source_bars(
            &client,
            source,
            requirement.interval,
            from_ms,
            to_ms,
            endpoints,
        )?;
        feeds.push(SourceFeed {
            source_id: source.id,
            interval: requirement.interval,
            bars,
        });
    }

    Ok(SourceRuntimeConfig {
        base_interval,
        feeds,
    })
}

fn fetch_source_bars(
    client: &Client,
    source: &DeclaredMarketSource,
    interval: Interval,
    from_ms: i64,
    to_ms: i64,
    endpoints: &ExchangeEndpoints,
) -> Result<Vec<Bar>, ExchangeFetchError> {
    match source.template {
        SourceTemplate::BinanceSpot => fetch_binance_bars(
            client,
            source,
            interval,
            from_ms,
            to_ms,
            &endpoints.binance_spot_base_url,
            "/api/v3/klines",
        ),
        SourceTemplate::BinanceUsdm => fetch_binance_bars(
            client,
            source,
            interval,
            from_ms,
            to_ms,
            &endpoints.binance_usdm_base_url,
            "/fapi/v1/klines",
        ),
        SourceTemplate::HyperliquidPerps => fetch_hyperliquid_bars(
            client,
            source,
            interval,
            from_ms,
            to_ms,
            &endpoints.hyperliquid_info_url,
            source.symbol.clone(),
        ),
        SourceTemplate::HyperliquidSpot => {
            let coin = resolve_hyperliquid_spot_coin(client, &source.symbol, endpoints)?;
            fetch_hyperliquid_bars(
                client,
                source,
                interval,
                from_ms,
                to_ms,
                &endpoints.hyperliquid_info_url,
                coin,
            )
        }
    }
}

fn fetch_binance_bars(
    client: &Client,
    source: &DeclaredMarketSource,
    interval: Interval,
    from_ms: i64,
    to_ms: i64,
    base_url: &str,
    path: &str,
) -> Result<Vec<Bar>, ExchangeFetchError> {
    let mut start_time = from_ms;
    let mut bars: Vec<Bar> = Vec::new();
    loop {
        let response = client
            .get(format!("{}{}", base_url.trim_end_matches('/'), path))
            .query(&[
                ("symbol", source.symbol.as_str()),
                ("interval", interval.as_str()),
                ("startTime", &start_time.to_string()),
                ("endTime", &to_ms.saturating_sub(1).to_string()),
                ("limit", &BINANCE_PAGE_LIMIT.to_string()),
            ])
            .send()
            .map_err(|err| request_failed(source, interval, err.to_string()))?;
        if response.status() != StatusCode::OK {
            return Err(request_failed(
                source,
                interval,
                format!("HTTP {}", response.status()),
            ));
        }
        let rows: Vec<Vec<JsonValue>> = response
            .json()
            .map_err(|err| malformed_response(source, interval, err.to_string()))?;
        if rows.is_empty() {
            break;
        }

        let mut last_open = None;
        for row in rows.iter() {
            let bar = parse_binance_row(source, interval, row)?;
            let open_time = bar.time as i64;
            if open_time < from_ms || open_time >= to_ms {
                continue;
            }
            if let Some(previous) = bars.last() {
                let previous_open = previous.time as i64;
                if open_time <= previous_open {
                    return Err(malformed_response(
                        source,
                        interval,
                        "non-increasing kline response".to_string(),
                    ));
                }
            }
            last_open = Some(open_time);
            bars.push(bar);
        }

        if rows.len() < BINANCE_PAGE_LIMIT {
            break;
        }
        let Some(last_open) = last_open else {
            break;
        };
        let Some(next_start) = interval.next_open_time(last_open) else {
            break;
        };
        if next_start >= to_ms {
            break;
        }
        start_time = next_start;
    }

    if bars.is_empty() {
        return Err(no_data(source, interval));
    }
    Ok(bars)
}

fn fetch_hyperliquid_bars(
    client: &Client,
    source: &DeclaredMarketSource,
    interval: Interval,
    from_ms: i64,
    to_ms: i64,
    info_url: &str,
    coin: String,
) -> Result<Vec<Bar>, ExchangeFetchError> {
    let mut start_time = from_ms;
    let mut bars: Vec<Bar> = Vec::new();
    loop {
        let response = client
            .post(info_url)
            .json(&serde_json::json!({
                "type": "candleSnapshot",
                "req": {
                    "coin": coin,
                    "interval": interval.as_str(),
                    "startTime": start_time,
                    "endTime": to_ms
                }
            }))
            .send()
            .map_err(|err| request_failed(source, interval, err.to_string()))?;
        if response.status() != StatusCode::OK {
            return Err(request_failed(
                source,
                interval,
                format!("HTTP {}", response.status()),
            ));
        }
        let rows: Vec<HyperliquidCandle> = response
            .json()
            .map_err(|err| malformed_response(source, interval, err.to_string()))?;
        if rows.is_empty() {
            break;
        }

        let mut last_open = None;
        for row in &rows {
            let bar = row.to_bar(source, interval)?;
            let open_time = bar.time as i64;
            if open_time < from_ms || open_time >= to_ms {
                continue;
            }
            if let Some(previous) = bars.last() {
                let previous_open = previous.time as i64;
                if open_time <= previous_open {
                    return Err(malformed_response(
                        source,
                        interval,
                        "non-increasing candle response".to_string(),
                    ));
                }
            }
            last_open = Some(open_time);
            bars.push(bar);
        }

        if rows.len() < HYPERLIQUID_PAGE_LIMIT {
            break;
        }
        let Some(last_open) = last_open else {
            break;
        };
        let Some(next_start) = interval.next_open_time(last_open) else {
            break;
        };
        if next_start >= to_ms {
            break;
        }
        start_time = next_start;
    }

    if bars.is_empty() {
        return Err(no_data(source, interval));
    }
    Ok(bars)
}

fn resolve_hyperliquid_spot_coin(
    client: &Client,
    symbol: &str,
    endpoints: &ExchangeEndpoints,
) -> Result<String, ExchangeFetchError> {
    let response = client
        .post(&endpoints.hyperliquid_info_url)
        .json(&serde_json::json!({ "type": "spotMeta" }))
        .send()
        .map_err(|err| ExchangeFetchError::RequestFailed {
            alias: "spotMeta".to_string(),
            template: SourceTemplate::HyperliquidSpot.as_str(),
            symbol: symbol.to_string(),
            interval: "",
            message: err.to_string(),
        })?;
    if response.status() != StatusCode::OK {
        return Err(ExchangeFetchError::RequestFailed {
            alias: "spotMeta".to_string(),
            template: SourceTemplate::HyperliquidSpot.as_str(),
            symbol: symbol.to_string(),
            interval: "",
            message: format!("HTTP {}", response.status()),
        });
    }
    let meta: HyperliquidSpotMeta =
        response
            .json()
            .map_err(|err| ExchangeFetchError::MalformedResponse {
                alias: "spotMeta".to_string(),
                template: SourceTemplate::HyperliquidSpot.as_str(),
                symbol: symbol.to_string(),
                interval: "",
                message: err.to_string(),
            })?;

    if meta
        .universe
        .iter()
        .any(|entry| entry.name.eq_ignore_ascii_case(symbol))
    {
        return Ok(symbol.to_string());
    }
    let canonical_pair = format!("{}/USDC", symbol.to_ascii_uppercase());
    if let Some(entry) = meta
        .universe
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case(&canonical_pair))
    {
        return Ok(entry.name.clone());
    }
    if let Some(token) = meta
        .tokens
        .iter()
        .find(|token| token.name.eq_ignore_ascii_case(symbol))
    {
        if let Some(entry) = meta
            .universe
            .iter()
            .find(|entry| entry.tokens.first().copied() == Some(token.index))
        {
            return Ok(entry.name.clone());
        }
    }
    Err(ExchangeFetchError::UnknownHyperliquidSpotSymbol {
        symbol: symbol.to_string(),
    })
}

fn parse_binance_row(
    source: &DeclaredMarketSource,
    interval: Interval,
    row: &[JsonValue],
) -> Result<Bar, ExchangeFetchError> {
    if row.len() < 6 {
        return Err(malformed_response(
            source,
            interval,
            "kline row is missing OHLCV fields".to_string(),
        ));
    }
    Ok(Bar {
        time: parse_json_i64(&row[0], source, interval, "open time")? as f64,
        open: parse_json_f64(&row[1], source, interval, "open")?,
        high: parse_json_f64(&row[2], source, interval, "high")?,
        low: parse_json_f64(&row[3], source, interval, "low")?,
        close: parse_json_f64(&row[4], source, interval, "close")?,
        volume: parse_json_f64(&row[5], source, interval, "volume")?,
    })
}

fn parse_json_i64(
    value: &JsonValue,
    source: &DeclaredMarketSource,
    interval: Interval,
    field: &str,
) -> Result<i64, ExchangeFetchError> {
    value
        .as_i64()
        .or_else(|| value.as_str().and_then(|text| text.parse::<i64>().ok()))
        .ok_or_else(|| malformed_response(source, interval, format!("invalid `{field}` value")))
}

fn parse_json_f64(
    value: &JsonValue,
    source: &DeclaredMarketSource,
    interval: Interval,
    field: &str,
) -> Result<f64, ExchangeFetchError> {
    value
        .as_f64()
        .or_else(|| value.as_str().and_then(|text| text.parse::<f64>().ok()))
        .ok_or_else(|| malformed_response(source, interval, format!("invalid `{field}` value")))
}

fn request_failed(
    source: &DeclaredMarketSource,
    interval: Interval,
    message: String,
) -> ExchangeFetchError {
    ExchangeFetchError::RequestFailed {
        alias: source.alias.clone(),
        template: source.template.as_str(),
        symbol: source.symbol.clone(),
        interval: interval.as_str(),
        message,
    }
}

fn malformed_response(
    source: &DeclaredMarketSource,
    interval: Interval,
    message: String,
) -> ExchangeFetchError {
    ExchangeFetchError::MalformedResponse {
        alias: source.alias.clone(),
        template: source.template.as_str(),
        symbol: source.symbol.clone(),
        interval: interval.as_str(),
        message,
    }
}

fn no_data(source: &DeclaredMarketSource, interval: Interval) -> ExchangeFetchError {
    ExchangeFetchError::NoData {
        alias: source.alias.clone(),
        template: source.template.as_str(),
        symbol: source.symbol.clone(),
        interval: interval.as_str(),
    }
}

#[derive(Clone, Debug, Deserialize)]
struct HyperliquidSpotMeta {
    universe: Vec<HyperliquidSpotUniverseEntry>,
    tokens: Vec<HyperliquidSpotToken>,
}

#[derive(Clone, Debug, Deserialize)]
struct HyperliquidSpotUniverseEntry {
    name: String,
    #[serde(default)]
    tokens: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize)]
struct HyperliquidSpotToken {
    name: String,
    index: usize,
}

#[derive(Clone, Debug, Deserialize)]
struct HyperliquidCandle {
    #[serde(rename = "t")]
    open_time: i64,
    #[serde(rename = "o")]
    open: String,
    #[serde(rename = "h")]
    high: String,
    #[serde(rename = "l")]
    low: String,
    #[serde(rename = "c")]
    close: String,
    #[serde(rename = "v")]
    volume: String,
}

impl HyperliquidCandle {
    fn to_bar(
        &self,
        source: &DeclaredMarketSource,
        interval: Interval,
    ) -> Result<Bar, ExchangeFetchError> {
        Ok(Bar {
            time: self.open_time as f64,
            open: self.open.parse().map_err(|_| {
                malformed_response(source, interval, "invalid `open` value".to_string())
            })?,
            high: self.high.parse().map_err(|_| {
                malformed_response(source, interval, "invalid `high` value".to_string())
            })?,
            low: self.low.parse().map_err(|_| {
                malformed_response(source, interval, "invalid `low` value".to_string())
            })?,
            close: self.close.parse().map_err(|_| {
                malformed_response(source, interval, "invalid `close` value".to_string())
            })?,
            volume: self.volume.parse().map_err(|_| {
                malformed_response(source, interval, "invalid `volume` value".to_string())
            })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        fetch_source_runtime_config, parse_binance_row, resolve_hyperliquid_spot_coin,
        ExchangeEndpoints, ExchangeFetchError,
    };
    use crate::compile;
    use crate::interval::{DeclaredMarketSource, Interval, SourceTemplate};
    use crate::runtime::Bar;
    use mockito::{Matcher, Server};
    use serde_json::json;

    fn sample_source(template: SourceTemplate, symbol: &str) -> DeclaredMarketSource {
        DeclaredMarketSource {
            id: 0,
            alias: "src".to_string(),
            template,
            symbol: symbol.to_string(),
        }
    }

    #[test]
    fn parse_binance_row_maps_ohlcv_fields() {
        let source = sample_source(SourceTemplate::BinanceSpot, "BTCUSDT");
        let bar = parse_binance_row(
            &source,
            Interval::Min1,
            &[
                json!(1704067200000_i64),
                json!("1.0"),
                json!("2.0"),
                json!("0.5"),
                json!("1.5"),
                json!("10.0"),
            ],
        )
        .expect("row parses");
        assert_eq!(
            bar,
            Bar {
                time: 1704067200000.0,
                open: 1.0,
                high: 2.0,
                low: 0.5,
                close: 1.5,
                volume: 10.0,
            }
        );
    }

    #[test]
    fn resolves_hyperliquid_spot_symbol_from_meta() {
        let mut server = Server::new();
        let _meta = server
            .mock("POST", "/info")
            .match_body(Matcher::Json(json!({ "type": "spotMeta" })))
            .with_status(200)
            .with_body(
                json!({
                    "universe": [{"name": "@107", "tokens": [107, 0]}],
                    "tokens": [{"name": "HYPE", "index": 107}]
                })
                .to_string(),
            )
            .create();

        let endpoints = ExchangeEndpoints {
            binance_spot_base_url: String::new(),
            binance_usdm_base_url: String::new(),
            hyperliquid_info_url: format!("{}/info", server.url()),
        };
        let client = reqwest::blocking::Client::new();
        let coin = resolve_hyperliquid_spot_coin(&client, "HYPE", &endpoints).expect("coin");
        assert_eq!(coin, "@107");
    }

    #[test]
    fn fetch_source_runtime_config_builds_all_required_feeds() {
        let mut server = Server::new();
        let _binance = server
            .mock("GET", "/api/v3/klines")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("symbol".into(), "BTCUSDT".into()),
                Matcher::UrlEncoded("interval".into(), "1m".into()),
            ]))
            .with_status(200)
            .with_body(
                json!([
                    [1704067200000_i64, "1.0", "2.0", "0.5", "1.5", "10.0"],
                    [1704067260000_i64, "2.0", "3.0", "1.5", "2.5", "11.0"]
                ])
                .to_string(),
            )
            .create();
        let _hyper_base = server
            .mock("POST", "/info")
            .match_body(Matcher::PartialJson(json!({
                "type": "candleSnapshot",
                "req": { "coin": "BTC", "interval": "1m" }
            })))
            .with_status(200)
            .with_body(
                json!([
                    {"t": 1704067200000_i64, "o": "10.0", "h": "11.0", "l": "9.0", "c": "10.5", "v": "5.0"},
                    {"t": 1704067260000_i64, "o": "10.5", "h": "12.0", "l": "10.0", "c": "11.5", "v": "6.0"}
                ])
                .to_string(),
            )
            .create();
        let _hyper_hour = server
            .mock("POST", "/info")
            .match_body(Matcher::PartialJson(json!({
                "type": "candleSnapshot",
                "req": { "coin": "BTC", "interval": "1h" }
            })))
            .with_status(200)
            .with_body(
                json!([
                    {"t": 1704067200000_i64, "o": "10.0", "h": "12.0", "l": "9.0", "c": "11.5", "v": "11.0"}
                ])
                .to_string(),
            )
            .create();

        let compiled = compile(
            "interval 1m\nsource bn = binance.spot(\"BTCUSDT\")\nsource hl = hyperliquid.perps(\"BTC\")\nuse hl 1h\nplot(bn.close - hl.1h.close)",
        )
        .expect("compile");
        let endpoints = ExchangeEndpoints {
            binance_spot_base_url: server.url(),
            binance_usdm_base_url: server.url(),
            hyperliquid_info_url: format!("{}/info", server.url()),
        };

        let config =
            fetch_source_runtime_config(&compiled, 1704067200000, 1704067320000, &endpoints)
                .expect("config");
        assert_eq!(config.base_interval, Interval::Min1);
        assert_eq!(config.feeds.len(), 3);
    }

    #[test]
    fn rejects_market_fetch_for_scripts_without_sources() {
        let compiled = compile("interval 1m\nplot(close)").expect("compile");
        let err = fetch_source_runtime_config(
            &compiled,
            1704067200000,
            1704067260000,
            &ExchangeEndpoints::default(),
        )
        .expect_err("missing sources should fail");
        assert!(matches!(err, ExchangeFetchError::MissingSources));
    }
}
