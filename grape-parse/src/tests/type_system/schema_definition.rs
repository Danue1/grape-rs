use crate::Parse;
use grape_ast::{Name, OperationType, RootOperationTypeDefinition, SchemaDefinition};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn base() {
    let source = "schema { query: foo mutation: bar subscription: baz }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.schema_definition(),
        Ok(Some(SchemaDefinition {
            span: Span::new(0, 53),
            description: None,
            directives: vec![],
            fields: vec![
                RootOperationTypeDefinition {
                    span: Span::new(9, 19),
                    key: OperationType::Query,
                    value: Name {
                        span: Span::new(16, 19),
                        symbol: interner.intern("foo"),
                    },
                },
                RootOperationTypeDefinition {
                    span: Span::new(20, 33),
                    key: OperationType::Mutation,
                    value: Name {
                        span: Span::new(30, 33),
                        symbol: interner.intern("bar"),
                    },
                },
                RootOperationTypeDefinition {
                    span: Span::new(34, 51),
                    key: OperationType::Subscription,
                    value: Name {
                        span: Span::new(48, 51),
                        symbol: interner.intern("baz"),
                    },
                },
            ],
        })),
    );
}
