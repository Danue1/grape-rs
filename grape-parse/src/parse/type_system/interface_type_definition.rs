use crate::{Error, Parse};
use grape_ast::{InterfaceTypeDefinition, StringValue};
use grape_symbol::INTERFACE;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn interface_type_definition(&mut self) -> Result<Option<InterfaceTypeDefinition>, Error> {
        let description = self.string_value().ok();

        self.interface_type_definition_with_description(&description)
    }

    pub fn interface_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<InterfaceTypeDefinition>, Error> {
        if let (start_span, TokenKind::Name(INTERFACE)) = self.current() {
            let start_span = if let Some(description) = description {
                description.span
            } else {
                *start_span
            };

            self.bump();

            let name = self.name()?;
            let implement_interfaces = self.implement_interfaces()?;
            let directives = self.directives()?;
            let fields = self.field_definitions()?;

            let end_span = if let Some(field) = fields.last() {
                field.span
            } else if let Some(directive) = directives.last() {
                directive.span
            } else if let Some(interface) = implement_interfaces.last() {
                interface.span
            } else {
                name.span
            };
            let span = start_span.with_end(&end_span);

            Ok(Some(InterfaceTypeDefinition {
                span,
                description: description.clone(),
                name,
                implement_interfaces,
                directives,
                fields,
            }))
        } else {
            Ok(None)
        }
    }
}
