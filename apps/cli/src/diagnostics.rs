use std::path::Path;

use palmscript::{CompileError, RuntimeError};

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

fn diagnostic_kind_label(kind: palmscript::DiagnosticKind) -> &'static str {
    match kind {
        palmscript::DiagnosticKind::Lex => "lex",
        palmscript::DiagnosticKind::Parse => "parse",
        palmscript::DiagnosticKind::Type => "type",
        palmscript::DiagnosticKind::Compile => "compile",
    }
}

#[cfg(test)]
mod tests {
    use super::{format_compile_error, format_runtime_error};
    use palmscript::bytecode::OpCode;
    use palmscript::span::{Position, Span};
    use palmscript::{CompileError, Diagnostic, DiagnosticKind, RuntimeError};
    use std::path::Path;

    #[test]
    fn compile_error_formatter_includes_path_span_and_kind() {
        let error = CompileError::new(vec![Diagnostic::new(
            DiagnosticKind::Parse,
            "expected expression",
            Span::new(Position::new(1, 2, 3), Position::new(2, 2, 4)),
        )]);
        let rendered = format_compile_error(Path::new("strategy.ps"), &error);
        assert!(rendered.contains("compile failed for `strategy.ps`"));
        assert!(rendered.contains("strategy.ps:2:3: parse: expected expression"));
    }

    #[test]
    fn runtime_formatter_prefixes_messages() {
        let runtime = format_runtime_error(&RuntimeError::StackUnderflow {
            pc: 2,
            opcode: OpCode::Add,
        });
        assert!(runtime.starts_with("runtime error:"));
    }
}
