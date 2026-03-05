use tradelang::compile;

fn compile_err(source: &str) -> String {
    let err = compile(source).expect_err("expected compile error");
    err.diagnostics
        .into_iter()
        .map(|diag| diag.message)
        .collect::<Vec<_>>()
        .join(" | ")
}

#[test]
fn rejects_missing_right_bracket() {
    let message = compile_err("plot(close[1)");
    assert!(message.contains("expected `]` after index"));
}

#[test]
fn rejects_negative_index() {
    let message = compile_err("plot(close[-1])");
    assert!(message.contains("series indexing requires a non-negative integer literal"));
}

#[test]
fn rejects_non_literal_index() {
    let message = compile_err("let n = 1\nplot(close[n])");
    assert!(message.contains("series indexing requires a non-negative integer literal"));
}

#[test]
fn rejects_non_literal_window_length() {
    let message = compile_err("let n = 14\nplot(sma(close, n))");
    assert!(message.contains("sma length must be a non-negative integer literal"));
}

#[test]
fn rejects_if_without_else() {
    let message = compile_err("if true { plot(1) }");
    assert!(message.contains("expected `else` after `if` block"));
}

#[test]
fn allows_shadowing_in_inner_scope() {
    compile("let x = close\nif close > close[1] { let x = close[1]\nplot(x) } else { plot(x) }")
        .expect("shadowing should compile");
}

#[test]
fn parses_na_literal() {
    compile("plot(na)").expect("na literal should compile");
}

#[test]
fn supports_newline_and_semicolon_separators() {
    compile("let x = close;\nplot(x)").expect("mixed separators should compile");
}
