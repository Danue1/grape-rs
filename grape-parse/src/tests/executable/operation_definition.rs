use crate::Parse;
use grape_ast::{Field, Name, OperationDefinition, OperationType, Selection};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn sugar() {
    let source = "{ foo }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.operation_definition(),
        Ok(Some(OperationDefinition {
            span: Span::new(0, 7),
            operation_type: OperationType::Query,
            name: None,
            variables: vec![],
            directives: vec![],
            selections: vec![Selection::Field(Field {
                span: Span::new(2, 5),
                alias: None,
                name: Name {
                    span: Span::new(2, 5),
                    symbol: interner.intern("foo"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![],
            })],
        })),
    );
}

#[test]
fn query() {
    let source = "query { foo }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.operation_definition(),
        Ok(Some(OperationDefinition {
            span: Span::new(0, 13),
            operation_type: OperationType::Query,
            name: None,
            variables: vec![],
            directives: vec![],
            selections: vec![Selection::Field(Field {
                span: Span::new(8, 11),
                alias: None,
                name: Name {
                    span: Span::new(8, 11),
                    symbol: interner.intern("foo"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![],
            })],
        })),
    );
}

#[test]
fn mutation() {
    let source = "mutation { foo }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.operation_definition(),
        Ok(Some(OperationDefinition {
            span: Span::new(0, 16),
            operation_type: OperationType::Mutation,
            name: None,
            variables: vec![],
            directives: vec![],
            selections: vec![Selection::Field(Field {
                span: Span::new(11, 14),
                alias: None,
                name: Name {
                    span: Span::new(11, 14),
                    symbol: interner.intern("foo"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![],
            })],
        })),
    );
}

#[test]
fn subscription() {
    let source = "subscription { foo }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.operation_definition(),
        Ok(Some(OperationDefinition {
            span: Span::new(0, 20),
            operation_type: OperationType::Subscription,
            name: None,
            variables: vec![],
            directives: vec![],
            selections: vec![Selection::Field(Field {
                span: Span::new(15, 18),
                alias: None,
                name: Name {
                    span: Span::new(15, 18),
                    symbol: interner.intern("foo"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![],
            })],
        })),
    );
}
