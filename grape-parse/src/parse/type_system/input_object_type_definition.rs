use crate::Parse;
use grape_ast::{InputObjectTypeDefinition, StringValue};
use grape_diagnostics::Message;
use grape_symbol::INPUT;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn input_object_type_definition(
        &mut self,
    ) -> Result<Option<InputObjectTypeDefinition>, Message> {
        let description = self.string_value().ok();

        self.input_object_type_definition_with_description(&description)
    }

    pub fn input_object_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<InputObjectTypeDefinition>, Message> {
        if let (start_span, TokenKind::Name(INPUT)) = self.current() {
            let start_span = if let Some(description) = description {
                description.span
            } else {
                *start_span
            };

            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;
            let (end_span, fields) =
                if let Some((end_span, fields)) = self.input_value_definitions_with_brace()? {
                    (end_span, fields)
                } else if let Some(directive) = directives.last() {
                    (directive.span, vec![])
                } else {
                    (name.span, vec![])
                };
            let span = start_span.with_end(&end_span);

            Ok(Some(InputObjectTypeDefinition {
                span,
                description: description.clone(),
                name,
                directives,
                fields,
            }))
        } else {
            Ok(None)
        }
    }
}
