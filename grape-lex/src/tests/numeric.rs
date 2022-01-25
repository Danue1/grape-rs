use crate::Lex;
use grape_span::Span;
use grape_symbol::SymbolInterner;
use grape_token::{Token, TokenKind};

#[test]
fn integer() {
    macro_rules! assert_integers {
        ($($source:expr => ($start:expr, $end:expr),)+) => {
            let source = "0 1 123456789 -0 -1 -123456789";
            let mut lex = Lex::new(source);
            let mut interner = SymbolInterner::new();

            $(
                assert_eq!(
                    lex.next(),
                    Some(Ok(Token {
                        span: Span::new($start, $end),
                        kind: TokenKind::Integer(interner.intern($source)),
                    })),
                );
            )+
        };
    }

    assert_integers! {
        "0" => (0, 1),
        "1" => (2, 3),
        "123456789" => (4, 13),

        "-0" => (14, 16),
        "-1" => (17, 19),
        "-123456789" => (20, 30),
    };
}

#[test]
fn float() {
    macro_rules! assert_float {
        ($($source:expr => ($start:expr, $end:expr),)+) => {
            let source = "0.0 123456789.123456789 -0.0 -123456789.123456789 \
                        0e0 1E1 2e+2 3E+3 4e-4 5E-5 \
                        -0e0 -1E1 -2e+2 -3E+3 -4e-4 -5E-5 \
                        0.0e0 1.1E1 2.2e+2 3.3E+3 4.4e-4 5.5E-5 \
                        -0.0e0 -1.1E1 -2.2e+2 -3.3E+3 -4.4e-4 -5.5E-5";
            let mut lex = Lex::new(source);
            let mut interner = SymbolInterner::new();

            $(
                assert_eq!(
                    lex.next(),
                    Some(Ok(Token {
                        span: Span::new($start, $end),
                        kind: TokenKind::Float(interner.intern($source)),
                    })),
                );
            )+
        };
    }

    assert_float![
        "0.0" => (0, 3),
        "123456789.123456789" => (4, 23),
        "-0.0" => (24, 28),
        "-123456789.123456789" => (29, 49),

        "0e0" => (50, 53),
        "1E1" => (54, 57),
        "2e+2" => (58, 62),
        "3E+3" => (63, 67),
        "4e-4" => (68, 72),
        "5E-5" => (73, 77),

        "-0e0" => (78, 82),
        "-1E1" => (83, 87),
        "-2e+2" => (88, 93),
        "-3E+3" => (94, 99),
        "-4e-4" => (100, 105),
        "-5E-5" => (106, 111),

        "0.0e0" => (112, 117),
        "1.1E1" => (118, 123),
        "2.2e+2" => (124, 130),
        "3.3E+3" => (131, 137),
        "4.4e-4" => (138, 144),
        "5.5E-5" => (145, 151),

        "-0.0e0" => (152, 158),
        "-1.1E1" => (159, 165),
        "-2.2e+2" => (166, 173),
        "-3.3E+3" => (174, 181),
        "-4.4e-4" => (182, 189),
        "-5.5E-5" => (190, 197),
    ];
}
