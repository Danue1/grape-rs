use crate::Lex;
use grape_span::Span;
use grape_symbol::SymbolInterner;
use grape_token::{Token, TokenKind};

#[test]
fn name() {
    macro_rules! assert_names {
        ($($source:expr => ($start:expr, $end:expr),)+) => {
            let source = "   a a1 _ _a _1 _a1 _1a";
            let mut lex = Lex::new(source);
            let mut interner = SymbolInterner::new();

            $(
                assert_eq!(
                    lex.next(),
                    Some(Ok(Token {
                        span: Span::new($start, $end),
                        kind: TokenKind::Name(interner.intern($source))
                    }))
                );
            )+
        };
    }

    assert_names! {
        "a" => (3, 4),
        "a1" => (5, 7),
        "_" => (8, 9),
        "_a" => (10, 12),
        "_1" => (13, 15),
        "_a1" => (16, 19),
        "_1a" => (20, 23),
    };
}
