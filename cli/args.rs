use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use tradelang::Interval;

#[derive(Debug, Parser)]
#[command(name = "tradelang")]
#[command(about = "TradeLang CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Run(RunArgs),
    Check(CheckArgs),
    DumpBytecode(DumpBytecodeArgs),
}

#[derive(Debug, clap::Args)]
pub struct RunArgs {
    pub script: PathBuf,
    #[arg(long)]
    pub bars: PathBuf,
    #[arg(long = "feed", value_parser = parse_feed_arg)]
    pub feeds: Vec<FeedArg>,
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
    #[arg(long, default_value_t = 10_000)]
    pub max_instructions_per_bar: usize,
    #[arg(long, default_value_t = 1_024)]
    pub max_history_capacity: usize,
}

#[derive(Debug, clap::Args)]
pub struct CheckArgs {
    pub script: PathBuf,
    #[arg(long)]
    pub env: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct DumpBytecodeArgs {
    pub script: PathBuf,
    #[arg(long)]
    pub env: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = BytecodeFormat::Text)]
    pub format: BytecodeFormat,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Json,
    Text,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum BytecodeFormat {
    #[default]
    Text,
    Json,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeedArg {
    pub interval: Interval,
    pub path: PathBuf,
}

pub fn parse_interval(raw: &str) -> Result<Interval, String> {
    Interval::parse(raw).ok_or_else(|| format!("invalid interval `{raw}`"))
}

pub fn parse_feed_arg(raw: &str) -> Result<FeedArg, String> {
    let Some((interval_text, path_text)) = raw.split_once('=') else {
        return Err("feed must use <interval=path>".to_string());
    };
    if path_text.is_empty() {
        return Err("feed path must not be empty".to_string());
    }
    Ok(FeedArg {
        interval: parse_interval(interval_text)?,
        path: PathBuf::from(path_text),
    })
}
