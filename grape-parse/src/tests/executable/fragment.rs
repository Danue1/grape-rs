use crate::Parse;
use grape_ast::{Directive, Field, FragmentDefinition, Name, Selection};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn without_directives() {
    let source = "fragment foo on bar { baz }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.fragment_definition(),
        Ok(Some(FragmentDefinition {
            span: Span::new(0, 27),
            name: Name {
                span: Span::new(9, 12),
                symbol: interner.intern("foo"),
            },
            type_condition: Name {
                span: Span::new(16, 19),
                symbol: interner.intern("bar"),
            },
            directives: vec![],
            selections: vec![Selection::Field(Field {
                span: Span::new(22, 25),
                alias: None,
                name: Name {
                    span: Span::new(22, 25),
                    symbol: interner.intern("baz"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![],
            })],
        })),
    );
}

#[test]
fn with_directives() {
    let source = "fragment foo on bar @baz { bax }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.fragment_definition(),
        Ok(Some(FragmentDefinition {
            span: Span::new(0, 32),
            name: Name {
                span: Span::new(9, 12),
                symbol: interner.intern("foo"),
            },
            type_condition: Name {
                span: Span::new(16, 19),
                symbol: interner.intern("bar"),
            },
            directives: vec![Directive {
                span: Span::new(20, 24),
                name: Name {
                    span: Span::new(21, 24),
                    symbol: interner.intern("baz"),
                },
                arguments: vec![],
            }],
            selections: vec![Selection::Field(Field {
                span: Span::new(27, 30),
                alias: None,
                name: Name {
                    span: Span::new(27, 30),
                    symbol: interner.intern("bax"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![],
            })],
        })),
    );
}
