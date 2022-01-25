use crate::Parse;
use grape_ast::{
    DirectiveDefinition, DirectiveLocation, EnumValue, ExecutableDirectiveLocation,
    ExecutableDirectiveLocationKind, InputValueDefinition, Name, Type, TypeKind, Value,
};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn executable() {
    let source = "directive @foo on QUERY";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.directive_definition(),
        Ok(Some(DirectiveDefinition {
            span: Span::new(0, 23),
            description: None,
            name: Name {
                span: Span::new(11, 14),
                symbol: interner.intern("foo"),
            },
            arguments: vec![],
            is_repeatable: false,
            locations: vec![DirectiveLocation::Executable(ExecutableDirectiveLocation {
                span: Span::new(18, 23),
                kind: ExecutableDirectiveLocationKind::Query,
            })],
        })),
    );
}

#[test]
fn executable_repeatable() {
    let source = "directive @foo repeatable on QUERY";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.directive_definition(),
        Ok(Some(DirectiveDefinition {
            span: Span::new(0, 34),
            description: None,
            name: Name {
                span: Span::new(11, 14),
                symbol: interner.intern("foo"),
            },
            arguments: vec![],
            is_repeatable: true,
            locations: vec![DirectiveLocation::Executable(ExecutableDirectiveLocation {
                span: Span::new(29, 34),
                kind: ExecutableDirectiveLocationKind::Query,
            })],
        })),
    );
}

#[test]
fn executable_with_arguments() {
    let source = "directive @foo(bar: baz = bax) on QUERY";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.directive_definition(),
        Ok(Some(DirectiveDefinition {
            span: Span::new(0, 39),
            description: None,
            name: Name {
                span: Span::new(11, 14),
                symbol: interner.intern("foo"),
            },
            arguments: vec![InputValueDefinition {
                span: Span::new(15, 29),
                description: None,
                name: Name {
                    span: Span::new(15, 18),
                    symbol: interner.intern("bar"),
                },
                ty: Type {
                    span: Span::new(20, 23),
                    kind: TypeKind::Named(Name {
                        span: Span::new(20, 23),
                        symbol: interner.intern("baz"),
                    }),
                },
                default_value: Some(Value::Enum(EnumValue {
                    span: Span::new(26, 29),
                    name: Name {
                        span: Span::new(26, 29),
                        symbol: interner.intern("bax"),
                    },
                })),
                directives: vec![],
            }],
            is_repeatable: false,
            locations: vec![DirectiveLocation::Executable(ExecutableDirectiveLocation {
                span: Span::new(34, 39),
                kind: ExecutableDirectiveLocationKind::Query,
            })],
        })),
    );
}
