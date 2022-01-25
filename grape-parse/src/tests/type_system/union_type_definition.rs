use crate::Parse;
use grape_ast::{Name, UnionTypeDefinition};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn base() {
    let source = "union foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.union_type_definition(),
        Ok(Some(UnionTypeDefinition {
            span: Span::new(0, 9),
            description: None,
            name: Name {
                span: Span::new(6, 9),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            members: vec![],
        })),
    );
}

#[test]
fn without_first_pipe() {
    let source = "union foo = bar | baz";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.union_type_definition(),
        Ok(Some(UnionTypeDefinition {
            span: Span::new(0, 21),
            description: None,
            name: Name {
                span: Span::new(6, 9),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            members: vec![
                Name {
                    span: Span::new(12, 15),
                    symbol: interner.intern("bar"),
                },
                Name {
                    span: Span::new(18, 21),
                    symbol: interner.intern("baz"),
                },
            ],
        })),
    );
}

#[test]
fn with_first_pipe() {
    let source = "union foo = | bar | baz";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.union_type_definition(),
        Ok(Some(UnionTypeDefinition {
            span: Span::new(0, 23),
            description: None,
            name: Name {
                span: Span::new(6, 9),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            members: vec![
                Name {
                    span: Span::new(14, 17),
                    symbol: interner.intern("bar"),
                },
                Name {
                    span: Span::new(20, 23),
                    symbol: interner.intern("baz"),
                },
            ],
        })),
    );
}
