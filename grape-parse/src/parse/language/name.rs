use crate::{Error, Parse};
use grape_ast::Name;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn name(&mut self) -> Result<Name, Error> {
        if let (span, TokenKind::Name(symbol)) = self.current() {
            let name = Name {
                span: *span,
                symbol: *symbol,
            };

            self.bump();

            Ok(name)
        } else {
            Err(Error::Unexpected)
        }
    }
}
