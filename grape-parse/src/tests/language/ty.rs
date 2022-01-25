use crate::Parse;
use grape_ast::{Name, Type, TypeKind};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn named() {
    let source = "foo";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.ty(),
        Ok(Type {
            span: Span::new(0, 3),
            kind: TypeKind::Named(Name {
                span: Span::new(0, 3),
                symbol: interner.intern("foo"),
            }),
        }),
    );
}

#[test]
fn nonnull_named() {
    let source = "foo!";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.ty(),
        Ok(Type {
            span: Span::new(0, 4),
            kind: TypeKind::NonNull(Box::new(Type {
                span: Span::new(0, 3),
                kind: TypeKind::Named(Name {
                    span: Span::new(0, 3),
                    symbol: interner.intern("foo"),
                }),
            }))
        }),
    );
}

#[test]
fn list() {
    let source = "[foo]";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.ty(),
        Ok(Type {
            span: Span::new(0, 5),
            kind: TypeKind::List(Box::new(Type {
                span: Span::new(1, 4),
                kind: TypeKind::Named(Name {
                    span: Span::new(1, 4),
                    symbol: interner.intern("foo"),
                })
            })),
        }),
    );
}

#[test]
fn list_nonnull_named() {
    let source = "[foo!]";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.ty(),
        Ok(Type {
            span: Span::new(0, 6),
            kind: TypeKind::List(Box::new(Type {
                span: Span::new(1, 5),
                kind: TypeKind::NonNull(Box::new(Type {
                    span: Span::new(1, 4),
                    kind: TypeKind::Named(Name {
                        span: Span::new(1, 4),
                        symbol: interner.intern("foo"),
                    })
                }))
            })),
        }),
    );
}

#[test]
fn nonnull_list_named() {
    let source = "[foo]!";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.ty(),
        Ok(Type {
            span: Span::new(0, 6),
            kind: TypeKind::NonNull(Box::new(Type {
                span: Span::new(0, 5),
                kind: TypeKind::List(Box::new(Type {
                    span: Span::new(1, 4),
                    kind: TypeKind::Named(Name {
                        span: Span::new(1, 4),
                        symbol: interner.intern("foo"),
                    })
                })),
            }))
        }),
    );
}

#[test]
fn nonnull_list_nonnull_named() {
    let source = "[foo!]!";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.ty(),
        Ok(Type {
            span: Span::new(0, 7),
            kind: TypeKind::NonNull(Box::new(Type {
                span: Span::new(0, 6),
                kind: TypeKind::List(Box::new(Type {
                    span: Span::new(1, 5),
                    kind: TypeKind::NonNull(Box::new(Type {
                        span: Span::new(1, 4),
                        kind: TypeKind::Named(Name {
                            span: Span::new(1, 4),
                            symbol: interner.intern("foo"),
                        })
                    }))
                })),
            }))
        }),
    );
}
