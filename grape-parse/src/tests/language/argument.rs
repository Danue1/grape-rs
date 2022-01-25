use crate::Parse;
use grape_ast::{Argument, EnumValue, ListValue, Name, Value};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn single() {
    let source = "(foo: bar)";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.arguments(),
        Ok(Some((
            Span::new(0, 10),
            vec![Argument {
                span: Span::new(1, 9),
                name: Name {
                    span: Span::new(1, 4),
                    symbol: interner.intern("foo"),
                },
                value: Value::Enum(EnumValue {
                    span: Span::new(6, 9),
                    name: Name {
                        span: Span::new(6, 9),
                        symbol: interner.intern("bar"),
                    },
                }),
            }],
        ))),
    );
}

#[test]
fn multi() {
    let source = "(foo: bar baz: [])";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.arguments(),
        Ok(Some((
            Span::new(0, 18),
            vec![
                Argument {
                    span: Span::new(1, 9),
                    name: Name {
                        span: Span::new(1, 4),
                        symbol: interner.intern("foo"),
                    },
                    value: Value::Enum(EnumValue {
                        span: Span::new(6, 9),
                        name: Name {
                            span: Span::new(6, 9),
                            symbol: interner.intern("bar"),
                        },
                    }),
                },
                Argument {
                    span: Span::new(10, 17),
                    name: Name {
                        span: Span::new(10, 13),
                        symbol: interner.intern("baz"),
                    },
                    value: Value::List(ListValue {
                        span: Span::new(15, 17),
                        values: vec![],
                    }),
                },
            ],
        ))),
    );
}
