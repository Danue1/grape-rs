use crate::Parse;
use grape_ast::{Argument, Directive, Name, NullValue, Value};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn without_arguments() {
    let source = "@foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.directives(),
        Ok(vec![Directive {
            span: Span::new(0, 4),
            name: Name {
                span: Span::new(1, 4),
                symbol: interner.intern("foo"),
            },
            arguments: vec![],
        }]),
    );
}

#[test]
fn with_argument() {
    let source = "@foo(bar: null)";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.directives(),
        Ok(vec![Directive {
            span: Span::new(0, 15),
            name: Name {
                span: Span::new(1, 4),
                symbol: interner.intern("foo"),
            },
            arguments: vec![Argument {
                span: Span::new(5, 14),
                name: Name {
                    span: Span::new(5, 8),
                    symbol: interner.intern("bar"),
                },
                value: Value::Null(NullValue {
                    span: Span::new(10, 14),
                })
            }],
        }]),
    );
}
