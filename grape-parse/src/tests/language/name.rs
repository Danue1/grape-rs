use crate::Parse;
use grape_ast::Name;
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn name() {
    let source = "hello world";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.name(),
        Ok(Name {
            span: Span::new(0, 5),
            symbol: interner.intern("hello"),
        }),
    );

    assert_eq!(
        parse.name(),
        Ok(Name {
            span: Span::new(6, 11),
            symbol: interner.intern("world"),
        }),
    );
}
