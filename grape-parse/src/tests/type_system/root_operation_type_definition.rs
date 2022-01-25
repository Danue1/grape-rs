use crate::Parse;
use grape_ast::{Name, OperationType, RootOperationTypeDefinition};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn base() {
    let source = "{ query: foo mutation: bar subscription: baz }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.root_operation_type_definitions(),
        Ok(Some((
            Span::new(0, 46),
            vec![
                RootOperationTypeDefinition {
                    span: Span::new(2, 12),
                    key: OperationType::Query,
                    value: Name {
                        span: Span::new(9, 12),
                        symbol: interner.intern("foo"),
                    },
                },
                RootOperationTypeDefinition {
                    span: Span::new(13, 26),
                    key: OperationType::Mutation,
                    value: Name {
                        span: Span::new(23, 26),
                        symbol: interner.intern("bar"),
                    },
                },
                RootOperationTypeDefinition {
                    span: Span::new(27, 44),
                    key: OperationType::Subscription,
                    value: Name {
                        span: Span::new(41, 44),
                        symbol: interner.intern("baz"),
                    },
                },
            ],
        ))),
    );
}
