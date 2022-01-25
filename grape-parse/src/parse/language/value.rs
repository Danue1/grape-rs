use crate::{expect, Error, Parse};
use grape_ast::{
    BooleanValue, EnumValue, FloatValue, IntValue, ListValue, Name, NullValue, ObjectField,
    ObjectValue, StringValue, Value, Variable,
};
use grape_symbol::{FALSE, NULL, TRUE};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn default_value(&mut self) -> Result<Option<Value>, Error> {
        if self.current_token() == &TokenKind::Equal {
            self.bump();
            self.value()
        } else {
            Ok(None)
        }
    }

    pub fn value(&mut self) -> Result<Option<Value>, Error> {
        match self.current() {
            (span, TokenKind::Name(NULL)) => {
                let value = Value::Null(NullValue { span: *span });

                self.bump();

                Ok(Some(value))
            }
            (span, TokenKind::Name(TRUE)) => {
                let value = Value::Boolean(BooleanValue {
                    span: *span,
                    value: true,
                });

                self.bump();

                Ok(Some(value))
            }
            (span, TokenKind::Name(FALSE)) => {
                let value = Value::Boolean(BooleanValue {
                    span: *span,
                    value: false,
                });

                self.bump();

                Ok(Some(value))
            }
            (span, TokenKind::Name(symbol)) => {
                let value = Value::Enum(EnumValue {
                    span: *span,
                    name: Name {
                        span: *span,
                        symbol: *symbol,
                    },
                });

                self.bump();

                Ok(Some(value))
            }
            (span, TokenKind::Dollar) => {
                let start_span = *span;

                self.bump();

                let name = self.name()?;

                Ok(Some(Value::Variable(Variable {
                    span: start_span.with_end(&name.span),
                    name,
                })))
            }
            (start_span, TokenKind::LeftBracket) => {
                let start_span = *start_span;

                self.bump();

                let mut values = vec![];

                while let Some(value) = self.value()? {
                    values.push(value);
                }

                if let (end_span, TokenKind::RightBracket) = self.current() {
                    let value = Value::List(ListValue {
                        span: start_span.with_end(end_span),
                        values,
                    });

                    self.bump();

                    Ok(Some(value))
                } else {
                    Err(Error::Unexpected)
                }
            }
            (start_span, TokenKind::LeftBrace) => {
                let start_span = *start_span;

                self.bump();

                let mut fields = vec![];

                while let Ok(key) = self.name() {
                    expect!(self, TokenKind::Colon);
                    if let Some(value) = self.value()? {
                        fields.push(ObjectField {
                            span: key.span.with_end(&value.span()),
                            key,
                            value,
                        });
                    } else {
                        return Err(Error::Unexpected);
                    }
                }

                if let (end_span, TokenKind::RightBrace) = self.current() {
                    let value = Value::Object(ObjectValue {
                        span: start_span.with_end(end_span),
                        fields,
                    });

                    self.bump();

                    Ok(Some(value))
                } else {
                    Err(Error::Unexpected)
                }
            }
            (span, TokenKind::Integer(symbol)) => {
                let value = Value::Int(IntValue {
                    span: *span,
                    symbol: *symbol,
                });

                self.bump();

                Ok(Some(value))
            }
            (span, TokenKind::Float(symbol)) => {
                let value = Value::Float(FloatValue {
                    span: *span,
                    symbol: *symbol,
                });

                self.bump();

                Ok(Some(value))
            }
            (span, TokenKind::String(symbol, is_block)) => {
                let value = Value::String(StringValue {
                    span: *span,
                    is_block: *is_block,
                    symbol: *symbol,
                });

                self.bump();

                Ok(Some(value))
            }
            _ => Ok(None),
        }
    }

    pub fn string_value(&mut self) -> Result<StringValue, Error> {
        if let (span, TokenKind::String(symbol, is_block)) = self.current() {
            let value = StringValue {
                span: *span,
                is_block: *is_block,
                symbol: *symbol,
            };

            self.bump();

            Ok(value)
        } else {
            Err(Error::Unexpected)
        }
    }
}
