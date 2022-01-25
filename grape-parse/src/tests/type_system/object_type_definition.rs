use crate::Parse;
use grape_ast::{FieldDefinition, Name, ObjectTypeDefinition, Type, TypeKind};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn base() {
    let source = "type foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.object_type_definition(),
        Ok(Some(ObjectTypeDefinition {
            span: Span::new(0, 8),
            description: None,
            name: Name {
                span: Span::new(5, 8),
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
    let source = "type foo implements bar & baz";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.object_type_definition(),
        Ok(Some(ObjectTypeDefinition {
            span: Span::new(0, 29),
            description: None,
            name: Name {
                span: Span::new(5, 8),
                symbol: interner.intern("foo"),
            },
            implement_interfaces: vec![
                Name {
                    span: Span::new(20, 23),
                    symbol: interner.intern("bar"),
                },
                Name {
                    span: Span::new(26, 29),
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
    let source = "type foo implements & bar & baz";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.object_type_definition(),
        Ok(Some(ObjectTypeDefinition {
            span: Span::new(0, 31),
            description: None,
            name: Name {
                span: Span::new(5, 8),
                symbol: interner.intern("foo"),
            },
            implement_interfaces: vec![
                Name {
                    span: Span::new(22, 25),
                    symbol: interner.intern("bar"),
                },
                Name {
                    span: Span::new(28, 31),
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
    let source = "type foo { bar: baz }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.object_type_definition(),
        Ok(Some(ObjectTypeDefinition {
            span: Span::new(0, 19),
            description: None,
            name: Name {
                span: Span::new(5, 8),
                symbol: interner.intern("foo"),
            },
            implement_interfaces: vec![],
            directives: vec![],
            fields: vec![FieldDefinition {
                span: Span::new(11, 19),
                description: None,
                name: Name {
                    span: Span::new(11, 14),
                    symbol: interner.intern("bar"),
                },
                arguments: vec![],
                ty: Type {
                    span: Span::new(16, 19),
                    kind: TypeKind::Named(Name {
                        span: Span::new(16, 19),
                        symbol: interner.intern("baz"),
                    }),
                },
                directives: vec![],
            }],
        })),
    );
}
