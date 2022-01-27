use crate::{error, spanned, Parse};
use grape_ast::{TypeSystemDefinitionOrExtension, TypeSystemExtensionDocument};
use grape_diagnostics::Message;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn type_system_extension_document(
        &mut self,
    ) -> Result<TypeSystemExtensionDocument, Message> {
        let mut definitions = vec![];
        while self.current_token() != &TokenKind::Eof {
            if let Some(definition) = self.type_system_definition()? {
                definitions.push(TypeSystemDefinitionOrExtension::Definition(definition));
            } else if let Some(definition) = self.type_system_extension()? {
                definitions.push(TypeSystemDefinitionOrExtension::Extension(definition))
            } else {
                error!();
            }
        }

        Ok(TypeSystemExtensionDocument {
            span: spanned!(definitions),
            definitions,
        })
    }
}
