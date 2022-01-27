use crate::Parse;
use grape_ast::{
    Argument, Directive, Field, FragmentSpread, InlineFragment, Name, NullValue, Selection, Value,
};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn name() {
    let source = "{ foo }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 7),
            vec![Selection::Field(Field {
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
        ))),
    );
}

#[test]
fn aliased_name() {
    let source = "{ foo: bar }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 12),
            vec![Selection::Field(Field {
                span: Span::new(2, 10),
                alias: Some(Name {
                    span: Span::new(2, 5),
                    symbol: interner.intern("foo"),
                }),
                name: Name {
                    span: Span::new(7, 10),
                    symbol: interner.intern("bar"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![],
            })],
        ))),
    );
}

#[test]
fn name_with_selections() {
    let source = "{ foo { bar } }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 15),
            vec![Selection::Field(Field {
                span: Span::new(2, 13),
                alias: None,
                name: Name {
                    span: Span::new(2, 5),
                    symbol: interner.intern("foo"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![Selection::Field(Field {
                    span: Span::new(8, 11),
                    alias: None,
                    name: Name {
                        span: Span::new(8, 11),
                        symbol: interner.intern("bar")
                    },
                    arguments: vec![],
                    directives: vec![],
                    selections: vec![],
                })],
            })],
        ))),
    );
}

#[test]
fn aliased_name_with_selections() {
    let source = "{ foo: bar { baz } }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 20),
            vec![Selection::Field(Field {
                span: Span::new(2, 18),
                alias: Some(Name {
                    span: Span::new(2, 5),
                    symbol: interner.intern("foo"),
                }),
                name: Name {
                    span: Span::new(7, 10),
                    symbol: interner.intern("bar"),
                },
                arguments: vec![],
                directives: vec![],
                selections: vec![Selection::Field(Field {
                    span: Span::new(13, 16),
                    alias: None,
                    name: Name {
                        span: Span::new(13, 16),
                        symbol: interner.intern("baz")
                    },
                    arguments: vec![],
                    directives: vec![],
                    selections: vec![],
                })],
            })],
        ))),
    );
}

#[test]
fn several_name() {
    let source = "{ foo bar baz }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 15),
            vec![
                Selection::Field(Field {
                    span: Span::new(2, 5),
                    alias: None,
                    name: Name {
                        span: Span::new(2, 5),
                        symbol: interner.intern("foo"),
                    },
                    arguments: vec![],
                    directives: vec![],
                    selections: vec![],
                }),
                Selection::Field(Field {
                    span: Span::new(6, 9),
                    alias: None,
                    name: Name {
                        span: Span::new(6, 9),
                        symbol: interner.intern("bar"),
                    },
                    arguments: vec![],
                    directives: vec![],
                    selections: vec![],
                }),
                Selection::Field(Field {
                    span: Span::new(10, 13),
                    alias: None,
                    name: Name {
                        span: Span::new(10, 13),
                        symbol: interner.intern("baz"),
                    },
                    arguments: vec![],
                    directives: vec![],
                    selections: vec![],
                })
            ],
        ))),
    );
}

#[test]
fn name_with_arguments() {
    let source = "{ foo(bar: null) }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 18),
            vec![Selection::Field(Field {
                span: Span::new(2, 5),
                alias: None,
                name: Name {
                    span: Span::new(2, 5),
                    symbol: interner.intern("foo"),
                },
                arguments: vec![Argument {
                    span: Span::new(6, 15),
                    key: Name {
                        span: Span::new(6, 9),
                        symbol: interner.intern("bar"),
                    },
                    value: Value::Null(NullValue {
                        span: Span::new(11, 15),
                    })
                }],
                directives: vec![],
                selections: vec![],
            })],
        ))),
    );
}

#[test]
fn name_with_directives() {
    let source = "{ foo @bar }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 12),
            vec![Selection::Field(Field {
                span: Span::new(2, 10),
                alias: None,
                name: Name {
                    span: Span::new(2, 5),
                    symbol: interner.intern("foo"),
                },
                arguments: vec![],
                directives: vec![Directive {
                    span: Span::new(6, 10),
                    name: Name {
                        span: Span::new(7, 10),
                        symbol: interner.intern("bar"),
                    },
                    arguments: vec![],
                }],
                selections: vec![],
            })],
        ))),
    );
}

#[test]
fn fragment_spread() {
    let source = "{ ...foo }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 10),
            vec![Selection::FragmentSpread(FragmentSpread {
                span: Span::new(2, 8),
                name: Name {
                    span: Span::new(5, 8),
                    symbol: interner.intern("foo"),
                },
                directives: vec![],
            })],
        ))),
    );
}

#[test]
fn fragment_spread_with_directives() {
    let source = "{ ...foo @bar }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 15),
            vec![Selection::FragmentSpread(FragmentSpread {
                span: Span::new(2, 13),
                name: Name {
                    span: Span::new(5, 8),
                    symbol: interner.intern("foo"),
                },
                directives: vec![Directive {
                    span: Span::new(9, 13),
                    name: Name {
                        span: Span::new(10, 13),
                        symbol: interner.intern("bar"),
                    },
                    arguments: vec![],
                }],
            })],
        ))),
    );
}

#[test]
fn inline_fragment() {
    let source = "{ ...on foo { bar } }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.selections(),
        Ok(Some((
            Span::new(0, 21),
            vec![Selection::InlineFragment(InlineFragment {
                span: Span::new(2, 19),
                name: Name {
                    span: Span::new(8, 11),
                    symbol: interner.intern("foo"),
                },
                directives: vec![],
                selections: vec![Selection::Field(Field {
                    span: Span::new(14, 17),
                    alias: None,
                    name: Name {
                        span: Span::new(14, 17),
                        symbol: interner.intern("bar"),
                    },
                    directives: vec![],
                    arguments: vec![],
                    selections: vec![],
                })],
            })],
        ))),
    );
}
