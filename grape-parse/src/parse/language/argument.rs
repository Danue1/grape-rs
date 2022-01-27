use crate::{error, expect, Parse};
use grape_ast::Argument;
use grape_diagnostics::Message;
use grape_span::Span;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn arguments(&mut self) -> Result<Option<(Span, Vec<Argument>)>, Message> {
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
                    _ => error!(),
                }
            }
            _ => Ok(None),
        }
    }

    fn argument(&mut self) -> Result<Option<Argument>, Message> {
        let name = match self.name() {
            Ok(name) => name,
            Err(_) => return Ok(None),
        };

        expect!(self, TokenKind::Colon);

        if let Some(value) = self.value()? {
            Ok(Some(Argument {
                span: name.span.with_end(&value.span()),
                key: name,
                value,
            }))
        } else {
            error!()
        }
    }
}
