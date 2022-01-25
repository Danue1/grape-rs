use crate::{spanned, Error, Parse};
use grape_ast::{TypeSystemDefinitionOrExtension, TypeSystemExtensionDocument};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn type_system_extension_document(&mut self) -> Result<TypeSystemExtensionDocument, Error> {
        let mut definitions = vec![];
        while self.current_token() != &TokenKind::Eof {
            if let Some(definition) = self.type_system_definition()? {
                definitions.push(TypeSystemDefinitionOrExtension::Definition(definition));
            } else if let Some(definition) = self.type_system_extension()? {
                definitions.push(TypeSystemDefinitionOrExtension::Extension(definition))
            } else {
                return Err(Error::Unexpected);
            }
        }

        Ok(TypeSystemExtensionDocument {
            span: spanned!(definitions),
            definitions,
        })
    }
}
