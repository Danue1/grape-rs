use crate::Parse;
use grape_ast::{
    EnumValue, InputObjectTypeDefinition, InputValueDefinition, Name, Type, TypeKind, Value,
};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn base() {
    let source = "input foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.input_object_type_definition(),
        Ok(Some(InputObjectTypeDefinition {
            span: Span::new(0, 9),
            description: None,
            name: Name {
                span: Span::new(6, 9),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            fields: vec![],
        })),
    );
}

#[test]
fn with_fields() {
    let source = "input foo { bar: baz = bax }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.input_object_type_definition(),
        Ok(Some(InputObjectTypeDefinition {
            span: Span::new(0, 28),
            description: None,
            name: Name {
                span: Span::new(6, 9),
                symbol: interner.intern("foo"),
            },
            directives: vec![],
            fields: vec![InputValueDefinition {
                span: Span::new(12, 26),
                description: None,
                name: Name {
                    span: Span::new(12, 15),
                    symbol: interner.intern("bar"),
                },
                ty: Type {
                    span: Span::new(17, 20),
                    kind: TypeKind::Named(Name {
                        span: Span::new(17, 20),
                        symbol: interner.intern("baz"),
                    }),
                },
                default_value: Some(Value::Enum(EnumValue {
                    span: Span::new(23, 26),
                    name: Name {
                        span: Span::new(23, 26),
                        symbol: interner.intern("bax"),
                    },
                })),
                directives: vec![],
            }],
        })),
    );
}
