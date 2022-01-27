use crate::{error, spanned, Parse};
use grape_ast::TypeSystemDocument;
use grape_diagnostics::Message;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn type_system_document(&mut self) -> Result<TypeSystemDocument, Message> {
        let mut definitions = vec![];
        while self.current_token() != &TokenKind::Eof {
            if let Some(definition) = self.type_system_definition()? {
                definitions.push(definition);
            } else {
                error!();
            }
        }

        Ok(TypeSystemDocument {
            span: spanned!(definitions),
            definitions,
        })
    }
}
