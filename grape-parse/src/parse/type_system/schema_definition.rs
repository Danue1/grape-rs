use crate::{Error, Parse};
use grape_ast::{SchemaDefinition, StringValue};
use grape_symbol::SCHEMA;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn schema_definition(&mut self) -> Result<Option<SchemaDefinition>, Error> {
        let description = self.string_value().ok();

        self.schema_definition_with_description(&description)
    }

    pub fn schema_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<SchemaDefinition>, Error> {
        if let (start_span, TokenKind::Name(SCHEMA)) = self.current() {
            let start_span = if let Some(description) = description {
                description.span
            } else {
                *start_span
            };

            self.bump();

            let directives = self.directives()?;
            if let Some((end_span, fields)) = self.root_operation_type_definitions()? {
                Ok(Some(SchemaDefinition {
                    span: start_span.with_end(&end_span),
                    description: description.clone(),
                    directives,
                    fields,
                }))
            } else {
                Err(Error::Unexpected)
            }
        } else {
            Ok(None)
        }
    }
}
