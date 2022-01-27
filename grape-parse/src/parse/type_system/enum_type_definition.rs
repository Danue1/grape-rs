use crate::Parse;
use grape_ast::{EnumTypeDefinition, StringValue};
use grape_diagnostics::Message;
use grape_symbol::ENUM;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn enum_type_definition(&mut self) -> Result<Option<EnumTypeDefinition>, Message> {
        let description = self.string_value().ok();

        self.enum_type_definition_with_description(&description)
    }

    pub fn enum_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<EnumTypeDefinition>, Message> {
        if let (start_span, TokenKind::Name(ENUM)) = self.current() {
            let start_span = if let Some(description) = description {
                description.span
            } else {
                *start_span
            };

            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;

            let (end_span, values) =
                if let Some((end_span, values)) = self.enum_value_definitions()? {
                    (end_span, values)
                } else if let Some(directive) = directives.last() {
                    (directive.span, vec![])
                } else {
                    (name.span, vec![])
                };

            Ok(Some(EnumTypeDefinition {
                span: start_span.with_end(&end_span),
                description: description.clone(),
                name,
                directives,
                values,
            }))
        } else {
            Ok(None)
        }
    }
}
