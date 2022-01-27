use crate::{error, Parse};
use grape_ast::Name;
use grape_diagnostics::Message;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn name(&mut self) -> Result<Name, Message> {
        if let (span, TokenKind::Name(symbol)) = self.current() {
            let name = Name {
                span: *span,
                symbol: *symbol,
            };

            self.bump();

            Ok(name)
        } else {
            error!()
        }
    }
}
