use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use hex::encode as hex_encode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{merge_bars, ExchangeFetchError, VenueRiskSnapshot};
use crate::interval::{Interval, SourceTemplate};
use crate::runtime::Bar;

const CACHE_VERSION: u32 = 1;
const HISTORICAL_CACHE_ENV_VAR: &str = "PALMSCRIPT_HISTORICAL_CACHE_DIR";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HistoricalBarFamily {
    Ohlcv,
    FundingRate,
    SourceMarkPrice,
    IndexPrice,
    PremiumIndex,
    Basis,
    PerpMarkPrice,
}

impl HistoricalBarFamily {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ohlcv => "ohlcv",
            Self::FundingRate => "funding_rate",
            Self::SourceMarkPrice => "source_mark_price",
            Self::IndexPrice => "index_price",
            Self::PremiumIndex => "premium_index",
            Self::Basis => "basis",
            Self::PerpMarkPrice => "perp_mark_price",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HistoricalRiskAccessMode {
    PublicOnly,
    SignedPreferred,
}

impl HistoricalRiskAccessMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::PublicOnly => "public_only",
            Self::SignedPreferred => "signed_preferred",
        }
    }

    pub(crate) fn binance_usdm() -> Self {
        let has_api_key = std::env::var("PALMSCRIPT_BINANCE_USDM_API_KEY").is_ok();
        let has_api_secret = std::env::var("PALMSCRIPT_BINANCE_USDM_API_SECRET").is_ok();
        if has_api_key && has_api_secret {
            Self::SignedPreferred
        } else {
            Self::PublicOnly
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HistoricalBarCacheKey {
    pub template: SourceTemplate,
    pub symbol: String,
    pub interval: Interval,
    pub family: HistoricalBarFamily,
    pub base_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HistoricalRiskCacheKey {
    pub template: SourceTemplate,
    pub symbol: String,
    pub access_mode: HistoricalRiskAccessMode,
    pub base_url: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct CachedWindow {
    from_ms: i64,
    to_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct HistoricalBarPayload {
    cache_version: u32,
    key: HistoricalBarCacheKey,
    covered_windows: Vec<CachedWindow>,
    bars: Vec<Bar>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct HistoricalRiskPayload {
    cache_version: u32,
    key: HistoricalRiskCacheKey,
    snapshot: VenueRiskSnapshot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HistoricalCache {
    root: PathBuf,
}

impl HistoricalCache {
    pub(crate) fn discover() -> Option<Self> {
        let root = default_historical_cache_root()?;
        if fs::create_dir_all(&root).is_err() {
            return None;
        }
        Some(Self { root })
    }

    #[cfg(test)]
    pub(crate) fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn bars<F, G>(
        &self,
        key: HistoricalBarCacheKey,
        from_ms: i64,
        to_ms: i64,
        mut fetch_missing: F,
        no_data_error: G,
    ) -> Result<Vec<Bar>, ExchangeFetchError>
    where
        F: FnMut(i64, i64) -> Result<Vec<Bar>, ExchangeFetchError>,
        G: FnOnce(i64, i64) -> ExchangeFetchError,
    {
        let path = self.bar_payload_path(&key);
        let mut payload = self
            .read_json::<HistoricalBarPayload>(&path)
            .filter(|value| value.cache_version == CACHE_VERSION && value.key == key)
            .unwrap_or_else(|| HistoricalBarPayload {
                cache_version: CACHE_VERSION,
                key: key.clone(),
                covered_windows: Vec::new(),
                bars: Vec::new(),
            });
        payload.covered_windows = normalize_windows(payload.covered_windows);
        payload.bars = normalize_bars(payload.bars);

        let missing_windows = compute_missing_windows(&payload.covered_windows, from_ms, to_ms);
        if !missing_windows.is_empty() {
            let mut merged = bars_to_map(std::mem::take(&mut payload.bars));
            for window in missing_windows {
                match fetch_missing(window.from_ms, window.to_ms) {
                    Ok(bars) => merge_bars(&mut merged, bars),
                    Err(ExchangeFetchError::NoData { .. }) => {}
                    Err(err) => return Err(err),
                }
                payload.covered_windows =
                    merge_window(payload.covered_windows, window.from_ms, window.to_ms);
            }
            payload.bars = merged.into_values().collect();
            self.write_json(&path, &payload);
        }

        let bars = slice_bars(&payload.bars, from_ms, to_ms);
        if bars.is_empty() {
            return Err(no_data_error(from_ms, to_ms));
        }
        Ok(bars)
    }

    pub(crate) fn load_risk_snapshot(
        &self,
        key: &HistoricalRiskCacheKey,
    ) -> Option<VenueRiskSnapshot> {
        let path = self.risk_payload_path(key);
        self.read_json::<HistoricalRiskPayload>(&path)
            .filter(|value| value.cache_version == CACHE_VERSION && value.key == *key)
            .map(|value| value.snapshot)
    }

    pub(crate) fn store_risk_snapshot(
        &self,
        key: HistoricalRiskCacheKey,
        snapshot: &VenueRiskSnapshot,
    ) {
        let path = self.risk_payload_path(&key);
        self.write_json(
            &path,
            &HistoricalRiskPayload {
                cache_version: CACHE_VERSION,
                key,
                snapshot: snapshot.clone(),
            },
        );
    }

    fn bar_payload_path(&self, key: &HistoricalBarCacheKey) -> PathBuf {
        self.root
            .join("bars")
            .join(key.family.as_str())
            .join(format!("{}.json", digest_key(key)))
    }

    fn risk_payload_path(&self, key: &HistoricalRiskCacheKey) -> PathBuf {
        self.root
            .join("risk")
            .join(key.access_mode.as_str())
            .join(format!("{}.json", digest_key(key)))
    }

    fn read_json<T: DeserializeOwned>(&self, path: &Path) -> Option<T> {
        let bytes = fs::read(path).ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    fn write_json<T: Serialize>(&self, path: &Path, value: &T) {
        let Some(parent) = path.parent() else {
            return;
        };
        if fs::create_dir_all(parent).is_err() {
            return;
        }
        let Ok(bytes) = serde_json::to_vec_pretty(value) else {
            return;
        };
        let temp_path = path.with_extension(format!("tmp-{}", std::process::id()));
        if fs::write(&temp_path, bytes).is_err() {
            let _ = fs::remove_file(&temp_path);
            return;
        }
        if fs::rename(&temp_path, path).is_err() {
            let _ = fs::remove_file(&temp_path);
        }
    }
}

fn default_historical_cache_root() -> Option<PathBuf> {
    if let Ok(path) = std::env::var(HISTORICAL_CACHE_ENV_VAR) {
        if !path.is_empty() {
            return Some(PathBuf::from(path));
        }
    }
    if let Ok(path) = std::env::var("XDG_CACHE_HOME") {
        return Some(PathBuf::from(path).join("palmscript").join("historical"));
    }
    let home = std::env::var("HOME").ok()?;
    Some(
        PathBuf::from(home)
            .join(".cache")
            .join("palmscript")
            .join("historical"),
    )
}

fn digest_key<T: Serialize>(key: &T) -> String {
    let bytes = serde_json::to_vec(key).expect("cache keys serialize");
    let digest = Sha256::digest(bytes);
    hex_encode(digest)
}

fn bars_to_map(bars: Vec<Bar>) -> BTreeMap<i64, Bar> {
    let mut merged = BTreeMap::new();
    merge_bars(&mut merged, bars);
    merged
}

fn normalize_bars(bars: Vec<Bar>) -> Vec<Bar> {
    bars_to_map(bars).into_values().collect()
}

fn slice_bars(bars: &[Bar], from_ms: i64, to_ms: i64) -> Vec<Bar> {
    bars.iter()
        .filter(|bar| {
            let open_time = bar.time as i64;
            open_time >= from_ms && open_time < to_ms
        })
        .cloned()
        .collect()
}

fn normalize_windows(mut windows: Vec<CachedWindow>) -> Vec<CachedWindow> {
    windows.sort_by_key(|window| (window.from_ms, window.to_ms));
    let mut merged = Vec::new();
    for window in windows {
        merged = merge_window(merged, window.from_ms, window.to_ms);
    }
    merged
}

fn merge_window(mut windows: Vec<CachedWindow>, from_ms: i64, to_ms: i64) -> Vec<CachedWindow> {
    if from_ms >= to_ms {
        return windows;
    }
    windows.push(CachedWindow { from_ms, to_ms });
    windows.sort_by_key(|window| (window.from_ms, window.to_ms));
    let mut merged: Vec<CachedWindow> = Vec::with_capacity(windows.len());
    for window in windows {
        if let Some(previous) = merged.last_mut() {
            if window.from_ms <= previous.to_ms {
                previous.to_ms = previous.to_ms.max(window.to_ms);
                continue;
            }
        }
        merged.push(window);
    }
    merged
}

fn compute_missing_windows(
    windows: &[CachedWindow],
    from_ms: i64,
    to_ms: i64,
) -> Vec<CachedWindow> {
    if from_ms >= to_ms {
        return Vec::new();
    }
    let mut missing = Vec::new();
    let mut cursor = from_ms;
    for window in windows {
        if window.to_ms <= cursor {
            continue;
        }
        if window.from_ms >= to_ms {
            break;
        }
        if window.from_ms > cursor {
            missing.push(CachedWindow {
                from_ms: cursor,
                to_ms: window.from_ms.min(to_ms),
            });
        }
        cursor = cursor.max(window.to_ms);
        if cursor >= to_ms {
            break;
        }
    }
    if cursor < to_ms {
        missing.push(CachedWindow {
            from_ms: cursor,
            to_ms,
        });
    }
    missing
}
