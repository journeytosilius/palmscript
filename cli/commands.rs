use std::fs;
use std::path::Path;

use tradelang::{
    compile, compile_with_env, run_multi_interval, CompileEnvironment, CompiledProgram,
    MultiIntervalConfig, RuntimeError, VmLimits,
};

use crate::args::{
    BytecodeFormat, CheckArgs, Cli, Command, DumpBytecodeArgs, OutputFormat, RunArgs,
};
use crate::data::{load_bars_csv, load_compile_env, load_interval_feed};
use crate::diagnostics::{format_compile_error, format_runtime_error};
use crate::format::{render_bytecode_text, render_outputs_text};

pub fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Run(args) => run_script(args),
        Command::Check(args) => check_script(args),
        Command::DumpBytecode(args) => dump_bytecode(args),
    }
}

fn run_script(args: RunArgs) -> Result<(), String> {
    let source = load_source(&args.script)?;
    let compiled = compile(&source).map_err(|err| format_compile_error(&args.script, &err))?;
    if !compiled.program.external_inputs.is_empty() {
        return Err(
            "scripts with external inputs must run through the future pipeline command".to_string(),
        );
    }
    let base_interval = compiled
        .program
        .base_interval
        .ok_or_else(|| "compiled strategy is missing a base interval declaration".to_string())?;

    let base_bars = load_bars_csv(&args.bars)?;
    let supplemental = args
        .feeds
        .iter()
        .map(|feed| load_interval_feed(feed.interval, &feed.path))
        .collect::<Result<Vec<_>, _>>()?;
    let outputs = run_multi_interval(
        &compiled,
        &base_bars,
        MultiIntervalConfig {
            base_interval,
            supplemental,
        },
        VmLimits {
            max_instructions_per_bar: args.max_instructions_per_bar,
            max_history_capacity: args.max_history_capacity,
        },
    )
    .map_err(|err| format_runtime_error(&err))?;

    match args.format {
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(&outputs).map_err(|err| err.to_string())?
        ),
        OutputFormat::Text => print!("{}", render_outputs_text(&outputs)),
    }
    Ok(())
}

fn check_script(args: CheckArgs) -> Result<(), String> {
    let source = load_source(&args.script)?;
    let env = match args.env {
        Some(path) => load_compile_env(&path)?,
        None => CompileEnvironment::default(),
    };
    compile_source(&source, &args.script, &env)?;
    println!("{}: ok", args.script.display());
    Ok(())
}

fn dump_bytecode(args: DumpBytecodeArgs) -> Result<(), String> {
    let source = load_source(&args.script)?;
    let env = match args.env {
        Some(path) => load_compile_env(&path)?,
        None => CompileEnvironment::default(),
    };
    let compiled = compile_source(&source, &args.script, &env)?;
    match args.format {
        BytecodeFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(&compiled).map_err(|err| err.to_string())?
        ),
        BytecodeFormat::Text => print!("{}", render_bytecode_text(&compiled)),
    }
    Ok(())
}

fn compile_source(
    source: &str,
    path: &Path,
    env: &CompileEnvironment,
) -> Result<CompiledProgram, String> {
    if env.external_inputs.is_empty() {
        compile(source).map_err(|err| format_compile_error(path, &err))
    } else {
        compile_with_env(source, env).map_err(|err| format_compile_error(path, &err))
    }
}

fn load_source(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| format!("failed to read `{}`: {err}", path.display()))
}

#[allow(dead_code)]
fn _runtime_error(_err: RuntimeError) -> String {
    unreachable!()
}
