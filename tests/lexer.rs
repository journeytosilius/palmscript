use tradelang::lexer::lex;
use tradelang::TokenKind;

#[test]
fn lexes_fn_keyword() {
    let tokens = lex("fn helper() = close > open").expect("source should lex");
    assert!(matches!(tokens[0].kind, TokenKind::Fn));
}

#[test]
fn reserves_fn_as_keyword() {
    let tokens = lex("let fn = 1").expect("source should lex");
    assert!(matches!(tokens[1].kind, TokenKind::Fn));
}
