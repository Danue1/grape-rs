use crate::{expect, Error, Parse};
use grape_ast::Argument;
use grape_span::Span;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn arguments(&mut self) -> Result<Option<(Span, Vec<Argument>)>, Error> {
        match self.current() {
            (start_span, TokenKind::LeftParens) => {
                let start_span = *start_span;

                self.bump();

                let mut arguments = vec![];

                while let Some(argument) = self.argument()? {
                    arguments.push(argument);
                }

                match self.current() {
                    (span, TokenKind::RightParens) => {
                        let span = start_span.with_end(span);

                        self.bump();

                        Ok(Some((span, arguments)))
                    }
                    _ => Err(Error::Unexpected),
                }
            }
            _ => Ok(None),
        }
    }

    fn argument(&mut self) -> Result<Option<Argument>, Error> {
        let name = match self.name() {
            Ok(name) => name,
            Err(_) => return Ok(None),
        };

        expect!(self, TokenKind::Colon);

        if let Some(value) = self.value()? {
            Ok(Some(Argument {
                span: name.span.with_end(&value.span()),
                name,
                value,
            }))
        } else {
            Err(Error::Unexpected)
        }
    }
}
