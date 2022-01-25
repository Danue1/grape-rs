use crate::{spanned, Error, Parse};
use grape_ast::{Definition, Document};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn document(&mut self) -> Result<Document, Error> {
        let mut definitions = vec![];
        while self.current_token() != &TokenKind::Eof {
            if let Some(definition) = self.operation_definition()? {
                definitions.push(Definition::Operation(definition));
            } else if let Some(definition) = self.fragment_definition()? {
                definitions.push(Definition::Fragment(definition));
            } else if let Some(definition) = self.type_system_definition()? {
                definitions.push(Definition::TypeSystem(definition));
            } else if let Some(definition) = self.type_system_extension()? {
                definitions.push(Definition::Extension(definition));
            } else {
                return Err(Error::Unexpected);
            }
        }

        Ok(Document {
            span: spanned!(definitions),
            definitions,
        })
    }
}
