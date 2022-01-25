use crate::{Error, Parse};
use grape_ast::TypeSystemExtension;
use grape_symbol::EXTEND;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn type_system_extension(&mut self) -> Result<Option<TypeSystemExtension>, Error> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            if let Some(extension) = self.schema_extension_with_extend(&start_span)? {
                Ok(Some(TypeSystemExtension::Schema(extension)))
            } else if let Some(extension) = self.type_extension_with_extend(&start_span)? {
                Ok(Some(TypeSystemExtension::Type(extension)))
            } else {
                Err(Error::Unexpected)
            }
        } else {
            Ok(None)
        }
    }
}
