use crate::Lex;
use grape_span::Span;
use grape_token::{Token, TokenKind};

#[test]
fn punctuator() {
    macro_rules! assert_tokens {
        ($($kind:ident => ($start:expr, $end:expr),)+) => {
            let source = "!$&()...:=@[]{|}";
            let mut lex = Lex::new(source);

            $(
                assert_eq!(lex.next(), Some(Ok(Token {
                    span: Span::new($start, $end),
                    kind: TokenKind::$kind,
                })));
            )+
            assert_eq!(lex.next(), None);
        };
    }

    assert_tokens![
        Exclamation => (0, 1),
        Dollar => (1, 2),
        Ampersand => (2, 3),
        LeftParens => (3, 4),
        RightParens => (4, 5),
        DotDotDot => (5, 8),
        Colon => (8, 9),
        Equal => (9, 10),
        At => (10, 11),
        LeftBracket => (11, 12),
        RightBracket => (12, 13),
        LeftBrace => (13, 14),
        Pipe => (14, 15),
        RightBrace => (15, 16),
    ];
}
