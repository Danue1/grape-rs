use crate::Parse;
use grape_ast::{Directive, EnumTypeDefinition, EnumValueDefinition, Name, StringValue};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn with_nothing() {
    let source = "enum foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.enum_type_definition(),
        Ok(Some(EnumTypeDefinition {
            span: Span::new(0, 8),
            description: None,
            name: Name {
                span: Span::new(5, 8),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            values: vec![],
        })),
    );
}

#[test]
fn with_description() {
    let source = "\"\"enum foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.enum_type_definition(),
        Ok(Some(EnumTypeDefinition {
            span: Span::new(0, 10),
            description: Some(StringValue {
                span: Span::new(0, 2),
                is_block: false,
                symbol: interner.intern("\"\""),
            }),
            name: Name {
                span: Span::new(7, 10),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            values: vec![],
        })),
    );
}

#[test]
fn with_directives() {
    let source = "enum foo @bar";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.enum_type_definition(),
        Ok(Some(EnumTypeDefinition {
            span: Span::new(0, 13),
            description: None,
            name: Name {
                span: Span::new(5, 8),
                symbol: interner.intern("foo"),
            },
            directives: vec![Directive {
                span: Span::new(9, 13),
                name: Name {
                    span: Span::new(10, 13),
                    symbol: interner.intern("bar")
                },
                arguments: vec![],
            }],
            values: vec![],
        })),
    );
}

#[test]
fn with_values() {
    let source = "enum foo { bar baz bax }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.enum_type_definition(),
        Ok(Some(EnumTypeDefinition {
            span: Span::new(0, 24),
            description: None,
            name: Name {
                span: Span::new(5, 8),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            values: vec![
                EnumValueDefinition {
                    span: Span::new(11, 14),
                    description: None,
                    name: Name {
                        span: Span::new(11, 14),
                        symbol: interner.intern("bar"),
                    },
                    directives: vec![],
                },
                EnumValueDefinition {
                    span: Span::new(15, 18),
                    description: None,
                    name: Name {
                        span: Span::new(15, 18),
                        symbol: interner.intern("baz"),
                    },
                    directives: vec![],
                },
                EnumValueDefinition {
                    span: Span::new(19, 22),
                    description: None,
                    name: Name {
                        span: Span::new(19, 22),
                        symbol: interner.intern("bax"),
                    },
                    directives: vec![],
                }
            ],
        })),
    );
}
