use crate::Parse;
use grape_ast::{
    BooleanValue, EnumValue, FloatValue, IntValue, ListValue, Name, NullValue, ObjectField,
    ObjectValue, StringValue, Value, Variable,
};
use grape_span::Span;
use grape_symbol::SymbolInterner;

#[test]
fn variable() {
    let source = "$hello";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Variable(Variable {
            span: Span::new(0, 6),
            name: Name {
                span: Span::new(1, 6),
                symbol: interner.intern("hello"),
            }
        }))),
    );
}

#[test]
fn variable_with_space() {
    let source = "$ hello";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Variable(Variable {
            span: Span::new(0, 7),
            name: Name {
                span: Span::new(2, 7),
                symbol: interner.intern("hello"),
            },
        }))),
    );
}

#[test]
fn int() {
    let source = "123";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Int(IntValue {
            span: Span::new(0, 3),
            symbol: interner.intern("123"),
        }))),
    );
}

#[test]
fn float() {
    let source = "123.456";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Float(FloatValue {
            span: Span::new(0, 7),
            symbol: interner.intern("123.456")
        }))),
    );
}

#[test]
fn empty_string() {
    let source = "\"\"";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::String(StringValue {
            span: Span::new(0, 2),
            is_block: false,
            symbol: interner.intern("\"\""),
        }))),
    );
}

#[test]
fn singleline_string() {
    let source = "\"Hello, World!\"";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::String(StringValue {
            span: Span::new(0, 15),
            is_block: false,
            symbol: interner.intern("\"Hello, World!\""),
        }))),
    );
}

#[test]
fn multiline_string() {
    let source = "\"\"\"Hello, World!\"\"\"";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::String(StringValue {
            span: Span::new(0, 19),
            is_block: true,
            symbol: interner.intern("\"\"\"Hello, World!\"\"\""),
        }))),
    );
}

#[test]
fn boolean() {
    let source = "true";
    let mut parse = Parse::new(source);

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Boolean(BooleanValue {
            span: Span::new(0, 4),
            value: true,
        })))
    );

    let source = "false";
    let mut parse = Parse::new(source);

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Boolean(BooleanValue {
            span: Span::new(0, 5),
            value: false,
        })))
    );
}

#[test]
fn null() {
    let source = "null";
    let mut parse = Parse::new(source);

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Null(NullValue {
            span: Span::new(0, 4),
        }))),
    );
}

#[test]
fn r#enum() {
    let source = "FooBar";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Enum(EnumValue {
            span: Span::new(0, 6),
            name: Name {
                span: Span::new(0, 6),
                symbol: interner.intern("FooBar"),
            },
        }))),
    );
}

#[test]
fn empty_object() {
    let source = "{}";
    let mut parse = Parse::new(source);

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Object(ObjectValue {
            span: Span::new(0, 2),
            fields: vec![],
        }))),
    );
}

#[test]
fn object() {
    let source = "{ a: 1 b: false c: { } }";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::Object(ObjectValue {
            span: Span::new(0, 24),
            fields: vec![
                ObjectField {
                    span: Span::new(2, 6),
                    key: Name {
                        span: Span::new(2, 3),
                        symbol: interner.intern("a"),
                    },
                    value: Value::Int(IntValue {
                        span: Span::new(5, 6),
                        symbol: interner.intern("1"),
                    }),
                },
                ObjectField {
                    span: Span::new(7, 15),
                    key: Name {
                        span: Span::new(7, 8),
                        symbol: interner.intern("b"),
                    },
                    value: Value::Boolean(BooleanValue {
                        span: Span::new(10, 15),
                        value: false,
                    }),
                },
                ObjectField {
                    span: Span::new(16, 22),
                    key: Name {
                        span: Span::new(16, 17),
                        symbol: interner.intern("c"),
                    },
                    value: Value::Object(ObjectValue {
                        span: Span::new(19, 22),
                        fields: vec![],
                    }),
                }
            ],
        }))),
    );
}

#[test]
fn empty_list() {
    let source = "[]";
    let mut parse = Parse::new(source);

    assert_eq!(
        parse.value(),
        Ok(Some(Value::List(ListValue {
            span: Span::new(0, 2),
            values: vec![],
        }))),
    );
}

#[test]
fn list() {
    let source =
        "[$foo null true false 123 456.789 \"Hello, World!\" \"\"\"Hello, World!\"\"\" []]";
    let mut parse = Parse::new(source);
    let mut interner = SymbolInterner::new();

    assert_eq!(
        parse.value(),
        Ok(Some(Value::List(ListValue {
            span: Span::new(0, 73),
            values: vec![
                Value::Variable(Variable {
                    span: Span::new(1, 5),
                    name: Name {
                        span: Span::new(2, 5),
                        symbol: interner.intern("foo"),
                    },
                }),
                Value::Null(NullValue {
                    span: Span::new(6, 10),
                }),
                Value::Boolean(BooleanValue {
                    span: Span::new(11, 15),
                    value: true,
                }),
                Value::Boolean(BooleanValue {
                    span: Span::new(16, 21),
                    value: false,
                }),
                Value::Int(IntValue {
                    span: Span::new(22, 25),
                    symbol: interner.intern("123"),
                }),
                Value::Float(FloatValue {
                    span: Span::new(26, 33),
                    symbol: interner.intern("123.456"),
                }),
                Value::String(StringValue {
                    span: Span::new(34, 49),
                    is_block: false,
                    symbol: interner.intern("\"Hello, World!\""),
                }),
                Value::String(StringValue {
                    span: Span::new(50, 69),
                    is_block: true,
                    symbol: interner.intern("\"\"\"Hello, World!\"\"\""),
                }),
                Value::List(ListValue {
                    span: Span::new(70, 72),
                    values: vec![],
                }),
            ],
        }))),
    );
}
