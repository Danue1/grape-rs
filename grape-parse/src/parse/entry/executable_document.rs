use crate::{spanned, Error, Parse};
use grape_ast::{ExecutableDefinition, ExecutableDocument};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn executable_document(&mut self) -> Result<ExecutableDocument, Error> {
        let mut definitions = vec![];
        while self.current_token() != &TokenKind::Eof {
            if let Some(definition) = self.operation_definition()? {
                definitions.push(ExecutableDefinition::Operation(definition));
            } else if let Some(definition) = self.fragment_definition()? {
                definitions.push(ExecutableDefinition::Fragment(definition));
            } else {
                return Err(Error::Unexpected);
            }
        }

        Ok(ExecutableDocument {
            span: spanned!(definitions),
            definitions,
        })
    }
}
