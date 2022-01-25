use crate::{Error, Parse};
use grape_ast::{ScalarTypeDefinition, StringValue};
use grape_symbol::SCALAR;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn scalar_type_definition(&mut self) -> Result<Option<ScalarTypeDefinition>, Error> {
        let description = self.string_value().ok();

        self.scalar_type_definition_with_description(&description)
    }

    pub fn scalar_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<ScalarTypeDefinition>, Error> {
        if let (start_span, TokenKind::Name(SCALAR)) = self.current() {
            let start_span = if let Some(description) = description {
                description.span
            } else {
                *start_span
            };

            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;

            let end_span = if let Some(directive) = directives.last() {
                directive.span
            } else {
                name.span
            };

            Ok(Some(ScalarTypeDefinition {
                span: start_span.with_end(&end_span),
                description: description.clone(),
                name,
                directives,
            }))
        } else {
            Ok(None)
        }
    }
}
