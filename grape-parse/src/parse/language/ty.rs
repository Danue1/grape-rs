use crate::{error, Parse};
use grape_ast::{Name, Type, TypeKind};
use grape_diagnostics::Message;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn ty(&mut self) -> Result<Type, Message> {
        match self.current() {
            (start_span, TokenKind::LeftBracket) => {
                let start_span = *start_span;

                self.bump();

                let kind = TypeKind::List(Box::new(self.ty()?));

                if let (end_span, TokenKind::RightBracket) = self.current() {
                    let ty = Type {
                        span: start_span.with_end(end_span),
                        kind,
                    };

                    self.bump();

                    if let (end_span, TokenKind::Exclamation) = self.current() {
                        let ty = Type {
                            span: start_span.with_end(end_span),
                            kind: TypeKind::NonNull(Box::new(ty)),
                        };

                        self.bump();

                        Ok(ty)
                    } else {
                        Ok(ty)
                    }
                } else {
                    error!()
                }
            }
            (start_span, TokenKind::Name(symbol)) => {
                let start_span = *start_span;
                let symbol = *symbol;

                self.bump();

                let ty = Type {
                    span: start_span,
                    kind: TypeKind::Named(Name {
                        span: start_span,
                        symbol,
                    }),
                };

                if let (end_span, TokenKind::Exclamation) = self.current() {
                    let ty = Type {
                        span: start_span.with_end(end_span),
                        kind: TypeKind::NonNull(Box::new(ty)),
                    };

                    self.bump();

                    Ok(ty)
                } else {
                    Ok(ty)
                }
            }
            _ => error!(),
        }
    }
}
