use std::path::Path;

use tradelang::{CompileError, RuntimeError};

pub fn format_compile_error(path: &Path, err: &CompileError) -> String {
    let mut rendered = Vec::with_capacity(err.diagnostics.len() + 1);
    rendered.push(format!("compile failed for `{}`", path.display()));
    for diagnostic in &err.diagnostics {
        rendered.push(format!(
            "{}:{}:{}: {}: {}",
            path.display(),
            diagnostic.span.start.line,
            diagnostic.span.start.column,
            diagnostic_kind_label(diagnostic.kind.clone()),
            diagnostic.message
        ));
    }
    rendered.join("\n")
}

pub fn format_runtime_error(err: &RuntimeError) -> String {
    format!("runtime error: {err}")
}

fn diagnostic_kind_label(kind: tradelang::DiagnosticKind) -> &'static str {
    match kind {
        tradelang::DiagnosticKind::Lex => "lex",
        tradelang::DiagnosticKind::Parse => "parse",
        tradelang::DiagnosticKind::Type => "type",
        tradelang::DiagnosticKind::Compile => "compile",
    }
}
