use crate::Parse;
use grape_ast::{Directive, Name, ScalarTypeDefinition, StringValue};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn with_nothing() {
    let source = "scalar foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.scalar_type_definition(),
        Ok(Some(ScalarTypeDefinition {
            span: Span::new(0, 10),
            description: None,
            name: Name {
                span: Span::new(7, 10),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
        })),
    );
}

#[test]
fn with_description() {
    let source = "\"\"scalar foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.scalar_type_definition(),
        Ok(Some(ScalarTypeDefinition {
            span: Span::new(0, 12),
            description: Some(StringValue {
                span: Span::new(0, 2),
                is_block: false,
                symbol: interner.intern("\"\""),
            }),
            name: Name {
                span: Span::new(9, 12),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
        })),
    );
}

#[test]
fn with_directives() {
    let source = "scalar foo @bar";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.scalar_type_definition(),
        Ok(Some(ScalarTypeDefinition {
            span: Span::new(0, 15),
            description: None,
            name: Name {
                span: Span::new(7, 10),
                symbol: interner.intern("foo"),
            },
            directives: vec![Directive {
                span: Span::new(11, 15),
                name: Name {
                    span: Span::new(12, 15),
                    symbol: interner.intern("bar"),
                },
                arguments: vec![],
            }],
        })),
    );
}
