use crate::{Error, Parse};
use grape_ast::OperationType;
use grape_span::Span;
use grape_symbol::{MUTATION, QUERY, SUBSCRIPTION};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn operation_type(&mut self) -> Result<(Span, OperationType), Error> {
        match self.current() {
            (span, TokenKind::Name(QUERY)) => {
                let span = *span;

                self.bump();

                Ok((span, OperationType::Query))
            }
            (span, TokenKind::Name(MUTATION)) => {
                let span = *span;

                self.bump();

                Ok((span, OperationType::Mutation))
            }
            (span, TokenKind::Name(SUBSCRIPTION)) => {
                let span = *span;

                self.bump();

                Ok((span, OperationType::Subscription))
            }
            _ => Err(Error::Unexpected),
        }
    }
}
