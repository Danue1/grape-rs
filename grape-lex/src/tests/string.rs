use crate::Lex;
use grape_span::Span;
use grape_symbol::{SymbolInterner, EMPTY_STRING};
use grape_token::{Token, TokenKind};

#[test]
fn empty() {
    let source = "\"\"";
    let mut lex = Lex::new(source);

    assert_eq!(
        lex.next(),
        Some(Ok(Token {
            span: Span::new(0, 2),
            kind: TokenKind::String(EMPTY_STRING, false),
        }))
    );
}

#[test]
fn single() {
    let source = "\"Hello, World!\"";
    let mut lex = Lex::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        lex.next(),
        Some(Ok(Token {
            span: Span::new(0, 15),
            kind: TokenKind::String(interner.intern("\"Hello, World!\""), false),
        }))
    );
}

#[test]
fn multiline_oneline() {
    let source = "\"\"\"Hello, World!\"\"\"";
    let mut lex = Lex::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        lex.next(),
        Some(Ok(Token {
            span: Span::new(0, 19),
            kind: TokenKind::String(interner.intern("\"\"\"Hello, World!\"\"\""), true),
        }))
    );
}

#[test]
fn multiline_twoline() {
    let source = "\"\"\"Hello, World!\nHello, World!\"\"\"";
    let mut lex = Lex::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        lex.next(),
        Some(Ok(Token {
            span: Span::new(0, 33),
            kind: TokenKind::String(
                interner.intern("\"\"\"Hello, World!\nHello, World!\"\"\""),
                true
            ),
        }))
    );
}
