use crate::Parse;
use grape_ast::{FieldDefinition, InterfaceTypeDefinition, Name, Type, TypeKind};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn base() {
    let source = "interface foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.interface_type_definition(),
        Ok(Some(InterfaceTypeDefinition {
            span: Span::new(0, 13),
            description: None,
            name: Name {
                span: Span::new(10, 13),
                symbol: interner.intern("foo"),
            },
            implement_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        })),
    );
}

#[test]
fn with_implement_interfaces() {
    let source = "interface foo implements bar & baz";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.interface_type_definition(),
        Ok(Some(InterfaceTypeDefinition {
            span: Span::new(0, 34),
            description: None,
            name: Name {
                span: Span::new(10, 13),
                symbol: interner.intern("foo"),
            },
            implement_interfaces: vec![
                Name {
                    span: Span::new(25, 28),
                    symbol: interner.intern("bar"),
                },
                Name {
                    span: Span::new(31, 34),
                    symbol: interner.intern("baz"),
                },
            ],
            directives: vec![],
            fields: vec![],
        })),
    );
}

#[test]
fn with_implement_interfaces_with_first_ampersand() {
    let source = "interface foo implements & bar & baz";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.interface_type_definition(),
        Ok(Some(InterfaceTypeDefinition {
            span: Span::new(0, 36),
            description: None,
            name: Name {
                span: Span::new(10, 13),
                symbol: interner.intern("foo"),
            },
            implement_interfaces: vec![
                Name {
                    span: Span::new(27, 30),
                    symbol: interner.intern("bar"),
                },
                Name {
                    span: Span::new(33, 36),
                    symbol: interner.intern("baz"),
                },
            ],
            directives: vec![],
            fields: vec![],
        })),
    );
}

#[test]
fn with_fields() {
    let source = "interface foo { bar: baz }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.interface_type_definition(),
        Ok(Some(InterfaceTypeDefinition {
            span: Span::new(0, 26),
            description: None,
            name: Name {
                span: Span::new(10, 13),
                symbol: interner.intern("foo"),
            },
            implement_interfaces: vec![],
            directives: vec![],
            fields: vec![FieldDefinition {
                span: Span::new(16, 24),
                description: None,
                name: Name {
                    span: Span::new(16, 19),
                    symbol: interner.intern("bar"),
                },
                arguments: vec![],
                ty: Type {
                    span: Span::new(21, 24),
                    kind: TypeKind::Named(Name {
                        span: Span::new(21, 24),
                        symbol: interner.intern("baz"),
                    }),
                },
                directives: vec![],
            }],
        })),
    );
}
