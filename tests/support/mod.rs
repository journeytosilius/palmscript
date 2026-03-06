use palmscript::{compile, CompileError, DiagnosticKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpectedDiagnostic {
    pub kind: DiagnosticKind,
    pub message: &'static str,
}

pub fn compile_error(source: &str) -> CompileError {
    compile(source).expect_err("expected compile error")
}

pub fn compile_diagnostics(source: &str) -> Vec<(DiagnosticKind, String)> {
    compile_error(source)
        .diagnostics
        .into_iter()
        .map(|diagnostic| (diagnostic.kind, diagnostic.message))
        .collect()
}

pub fn assert_compile_diagnostics(name: &str, source: &str, expected: &[ExpectedDiagnostic]) {
    let actual = compile_diagnostics(source);
    let expected = expected
        .iter()
        .map(|diagnostic| (diagnostic.kind.clone(), diagnostic.message.to_string()))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "{name}");
}
