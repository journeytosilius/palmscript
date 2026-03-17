use chrono::{SecondsFormat, Utc};
use crossbeam_channel::{Receiver, Sender};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "trace" => Some(Self::Trace),
            "debug" => Some(Self::Debug),
            "info" => Some(Self::Info),
            "warn" | "warning" => Some(Self::Warn),
            "error" => Some(Self::Error),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LogFieldValue {
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
}

impl Serialize for LogFieldValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::I64(value) => serializer.serialize_i64(*value),
            Self::U64(value) => serializer.serialize_u64(*value),
            Self::F64(value) => serializer.serialize_f64(*value),
            Self::String(value) => serializer.serialize_str(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogField {
    pub key: String,
    pub value: LogFieldValue,
}

impl LogField {
    pub fn bool(key: impl Into<String>, value: bool) -> Self {
        Self {
            key: key.into(),
            value: LogFieldValue::Bool(value),
        }
    }

    pub fn i64(key: impl Into<String>, value: i64) -> Self {
        Self {
            key: key.into(),
            value: LogFieldValue::I64(value),
        }
    }

    pub fn u64(key: impl Into<String>, value: u64) -> Self {
        Self {
            key: key.into(),
            value: LogFieldValue::U64(value),
        }
    }

    pub fn f64(key: impl Into<String>, value: f64) -> Self {
        Self {
            key: key.into(),
            value: LogFieldValue::F64(value),
        }
    }

    pub fn string(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: LogFieldValue::String(value.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogRecord {
    pub ts: String,
    pub ts_ms: i64,
    pub level: Level,
    pub component: String,
    pub target: String,
    pub message: String,
    pub fields: Vec<LogField>,
}

impl LogRecord {
    pub fn new(
        level: Level,
        target: impl Into<String>,
        message: impl Into<String>,
        fields: Vec<LogField>,
    ) -> Self {
        let now = Utc::now();
        let target = target.into();
        Self {
            ts: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            ts_ms: now.timestamp_millis(),
            level,
            component: component_from_target(&target),
            target,
            message: message.into(),
            fields,
        }
    }

    pub fn with_component(
        level: Level,
        component: impl Into<String>,
        target: impl Into<String>,
        message: impl Into<String>,
        fields: Vec<LogField>,
    ) -> Self {
        let now = Utc::now();
        Self {
            ts: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            ts_ms: now.timestamp_millis(),
            level,
            component: component.into(),
            target: target.into(),
            message: message.into(),
            fields,
        }
    }
}

impl Serialize for LogRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("dt", &self.ts)?;
        map.serialize_entry("level", &self.level)?;
        map.serialize_entry("pid", &u64::from(*PROCESS_ID.get_or_init(std::process::id)))?;
        map.serialize_entry("hostname", resolve_hostname())?;
        map.serialize_entry("name", &resolve_log_name(&self.component))?;
        map.serialize_entry("module", &self.component)?;
        map.serialize_entry("eventType", &self.target)?;
        map.serialize_entry("message", &self.message)?;
        map.serialize_entry("ts", &self.ts)?;
        map.serialize_entry("ts_ms", &self.ts_ms)?;
        map.serialize_entry("component", &self.component)?;
        map.serialize_entry("target", &self.target)?;
        for field in &self.fields {
            map.serialize_entry(&field.key, &field.value)?;
        }
        map.end()
    }
}

pub trait LogSink: Send + Sync {
    fn emit(&self, record: &LogRecord);
}

pub struct JsonLineSink<W: Write + Send> {
    writer: Mutex<W>,
}

impl<W: Write + Send> JsonLineSink<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: Mutex::new(writer),
        }
    }
}

impl JsonLineSink<io::Stdout> {
    pub fn stdout() -> Self {
        Self::new(io::stdout())
    }
}

impl JsonLineSink<io::Stderr> {
    pub fn stderr() -> Self {
        Self::new(io::stderr())
    }
}

impl<W: Write + Send> LogSink for JsonLineSink<W> {
    fn emit(&self, record: &LogRecord) {
        let mut payload = match serde_json::to_vec(record) {
            Ok(payload) => payload,
            Err(_) => return,
        };
        payload.push(b'\n');
        let mut writer = self.writer.lock().expect("json line writer lock");
        if writer.write_all(&payload).is_ok() {
            let _ = writer.flush();
        }
    }
}

pub struct LogStore {
    inner: Mutex<VecDeque<LogRecord>>,
    capacity: usize,
}

impl LogStore {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(VecDeque::with_capacity(capacity.max(1))),
            capacity: capacity.max(1),
        }
    }

    pub fn push(&self, record: LogRecord) {
        let mut inner = self.inner.lock().expect("log store lock");
        if inner.len() >= self.capacity {
            inner.pop_front();
        }
        inner.push_back(record);
    }

    pub fn snapshot(&self) -> Vec<LogRecord> {
        let inner = self.inner.lock().expect("log store lock");
        inner.iter().cloned().collect()
    }
}

pub struct LogStoreSink {
    store: Arc<LogStore>,
}

impl LogStoreSink {
    pub fn new(store: Arc<LogStore>) -> Self {
        Self { store }
    }
}

impl LogSink for LogStoreSink {
    fn emit(&self, record: &LogRecord) {
        self.store.push(record.clone());
    }
}

#[derive(Clone)]
pub struct Logger {
    sender: Sender<LogRecord>,
    min_level: Level,
}

impl Logger {
    pub fn log(
        &self,
        level: Level,
        target: impl Into<String>,
        message: impl Into<String>,
        fields: Vec<LogField>,
    ) {
        if level < self.min_level {
            return;
        }
        let _ = self
            .sender
            .send(LogRecord::new(level, target, message, fields));
    }

    pub fn log_with_component(
        &self,
        level: Level,
        component: impl Into<String>,
        target: impl Into<String>,
        message: impl Into<String>,
        fields: Vec<LogField>,
    ) {
        if level < self.min_level {
            return;
        }
        let _ = self.sender.send(LogRecord::with_component(
            level, component, target, message, fields,
        ));
    }
}

pub struct LoggerGuard {
    join: Option<JoinHandle<()>>,
}

impl LoggerGuard {
    pub fn join(mut self) {
        if let Some(handle) = self.join.take() {
            let _ = handle.join();
        }
    }
}

pub struct LoggerBuilder {
    min_level: Level,
    sinks: Vec<Arc<dyn LogSink>>,
}

impl LoggerBuilder {
    pub fn new() -> Self {
        Self {
            min_level: default_level_from_env(),
            sinks: Vec::new(),
        }
    }

    pub fn min_level(mut self, level: Level) -> Self {
        self.min_level = level;
        self
    }

    pub fn add_sink(mut self, sink: Arc<dyn LogSink>) -> Self {
        self.sinks.push(sink);
        self
    }

    pub fn build(self) -> (Logger, LoggerGuard) {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut sinks = self.sinks;
        if let Some(sink) = BetterStackSink::from_env() {
            sinks.push(Arc::new(sink));
        }
        let join = thread::spawn(move || log_worker(receiver, sinks));
        (
            Logger {
                sender,
                min_level: self.min_level,
            },
            LoggerGuard { join: Some(join) },
        )
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum LoggerInitError {
    AlreadyInitialized,
}

static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();
static PROCESS_ID: OnceLock<u32> = OnceLock::new();
static HOSTNAME: OnceLock<String> = OnceLock::new();
static LOG_NAME: OnceLock<Option<String>> = OnceLock::new();

pub fn init_global(builder: LoggerBuilder) -> Result<LoggerGuard, LoggerInitError> {
    let (logger, guard) = builder.build();
    GLOBAL_LOGGER
        .set(logger)
        .map_err(|_| LoggerInitError::AlreadyInitialized)?;
    Ok(guard)
}

pub fn global() -> Option<&'static Logger> {
    GLOBAL_LOGGER.get()
}

pub fn log(
    level: Level,
    target: impl Into<String>,
    message: impl Into<String>,
    fields: Vec<LogField>,
) {
    if let Some(logger) = global() {
        logger.log(level, target, message, fields);
    }
}

pub fn log_with_component(
    level: Level,
    component: impl Into<String>,
    target: impl Into<String>,
    message: impl Into<String>,
    fields: Vec<LogField>,
) {
    if let Some(logger) = global() {
        logger.log_with_component(level, component, target, message, fields);
    }
}

pub fn trace(target: impl Into<String>, message: impl Into<String>) {
    log(Level::Trace, target, message, Vec::new());
}

pub fn debug(target: impl Into<String>, message: impl Into<String>) {
    log(Level::Debug, target, message, Vec::new());
}

pub fn info(target: impl Into<String>, message: impl Into<String>) {
    log(Level::Info, target, message, Vec::new());
}

pub fn warn(target: impl Into<String>, message: impl Into<String>) {
    log(Level::Warn, target, message, Vec::new());
}

pub fn error(target: impl Into<String>, message: impl Into<String>) {
    log(Level::Error, target, message, Vec::new());
}

pub fn info_fields(target: impl Into<String>, message: impl Into<String>, fields: Vec<LogField>) {
    log(Level::Info, target, message, fields);
}

pub fn debug_fields(target: impl Into<String>, message: impl Into<String>, fields: Vec<LogField>) {
    log(Level::Debug, target, message, fields);
}

pub fn warn_fields(target: impl Into<String>, message: impl Into<String>, fields: Vec<LogField>) {
    log(Level::Warn, target, message, fields);
}

pub fn error_fields(target: impl Into<String>, message: impl Into<String>, fields: Vec<LogField>) {
    log(Level::Error, target, message, fields);
}

#[derive(Clone)]
pub struct BetterStackSink {
    client: reqwest::blocking::Client,
    endpoint: String,
    source_token: String,
}

impl BetterStackSink {
    pub fn from_env() -> Option<Self> {
        let source_token = env_non_empty("BETTERSTACK_SOURCE_TOKEN")
            .or_else(|| env_non_empty("LOGTAIL_SOURCE_TOKEN"))?;
        let endpoint = env_non_empty("BETTERSTACK_LOGS_URL")
            .unwrap_or_else(|| "https://in.logs.betterstack.com".to_string());
        let timeout_ms = std::env::var("BETTERSTACK_TIMEOUT_MS")
            .ok()
            .and_then(|value| value.trim().parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(2_000);
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .ok()?;
        Some(Self {
            client,
            endpoint,
            source_token,
        })
    }
}

impl LogSink for BetterStackSink {
    fn emit(&self, record: &LogRecord) {
        let payload = match serde_json::to_vec(record) {
            Ok(payload) => payload,
            Err(_) => return,
        };
        let bearer = format!("Bearer {}", self.source_token);
        let _ = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", bearer)
            .header("X-Insert-Key", &self.source_token)
            .body(payload)
            .send();
    }
}

fn default_level_from_env() -> Level {
    std::env::var("PALMSCRIPT_LOG_LEVEL")
        .ok()
        .or_else(|| std::env::var("LOG_LEVEL").ok())
        .and_then(|value| Level::parse(&value))
        .unwrap_or(Level::Info)
}

fn component_from_target(target: &str) -> String {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return "unknown".to_string();
    }
    trimmed.split('.').next().unwrap_or(trimmed).to_string()
}

fn log_worker(receiver: Receiver<LogRecord>, sinks: Vec<Arc<dyn LogSink>>) {
    while let Ok(record) = receiver.recv() {
        for sink in &sinks {
            sink.emit(&record);
        }
    }
}

fn resolve_hostname() -> &'static str {
    HOSTNAME
        .get_or_init(|| {
            std::env::var("HOSTNAME")
                .or_else(|_| std::env::var("COMPUTERNAME"))
                .unwrap_or_else(|_| "unknown".to_string())
        })
        .as_str()
}

fn resolve_log_name(default_component: &str) -> String {
    LOG_NAME
        .get_or_init(|| {
            env_non_empty("PALMSCRIPT_LOG_NAME")
                .or_else(|| env_non_empty("LOGGER_NAME"))
                .or_else(|| env_non_empty("LOG_NAME"))
                .or_else(|| env_non_empty("APP_NAME"))
                .or_else(|| env_non_empty("SERVICE_NAME"))
        })
        .clone()
        .unwrap_or_else(|| default_component.to_string())
}

fn env_non_empty(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn record_uses_utc_timestamp() {
        let record = LogRecord::new(Level::Info, "test", "hello", Vec::new());
        assert!(record.ts.ends_with('Z'));
    }

    #[test]
    fn record_sets_component_from_target() {
        let record = LogRecord::new(Level::Info, "strategy.paper", "ok", Vec::new());
        assert_eq!(record.component, "strategy");
    }

    #[test]
    fn store_caps_capacity() {
        let store = LogStore::new(2);
        store.push(LogRecord::new(Level::Info, "test", "one", Vec::new()));
        store.push(LogRecord::new(Level::Info, "test", "two", Vec::new()));
        store.push(LogRecord::new(Level::Info, "test", "three", Vec::new()));
        let snapshot = store.snapshot();
        assert_eq!(snapshot.len(), 2);
        assert_eq!(snapshot[0].message, "two");
        assert_eq!(snapshot[1].message, "three");
    }

    #[test]
    fn logger_respects_min_level() {
        let store = Arc::new(LogStore::new(8));
        let (logger, guard) = LoggerBuilder::new()
            .min_level(Level::Info)
            .add_sink(Arc::new(LogStoreSink::new(store.clone())))
            .build();
        logger.log(Level::Debug, "test", "skip", Vec::new());
        logger.log(Level::Info, "test", "keep", Vec::new());
        drop(logger);
        guard.join();
        let snapshot = store.snapshot();
        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot[0].message, "keep");
    }

    #[test]
    fn record_serialization_flattens_fields() {
        let record = LogRecord::new(
            Level::Info,
            "paper.session",
            "updated",
            vec![
                LogField::string("session_id", "paper-1"),
                LogField::u64("trade_count", 2),
            ],
        );
        let value = serde_json::to_value(&record).expect("serialize record");
        assert_eq!(value["eventType"], "paper.session");
        assert_eq!(value["session_id"], "paper-1");
        assert_eq!(value["trade_count"], 2);
    }

    struct CountingWriter {
        writes: Arc<AtomicUsize>,
        bytes: Vec<u8>,
    }

    impl CountingWriter {
        fn new(writes: Arc<AtomicUsize>) -> Self {
            Self {
                writes,
                bytes: Vec::new(),
            }
        }
    }

    impl Write for CountingWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.writes.fetch_add(1, Ordering::SeqCst);
            self.bytes.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn json_line_sink_emits_a_single_write_per_record() {
        let writes = Arc::new(AtomicUsize::new(0));
        let sink = JsonLineSink::new(CountingWriter::new(writes.clone()));
        sink.emit(&LogRecord::new(
            Level::Info,
            "paper.session",
            "updated",
            vec![LogField::string("session_id", "paper-1")],
        ));
        assert_eq!(writes.load(Ordering::SeqCst), 1);
    }
}
