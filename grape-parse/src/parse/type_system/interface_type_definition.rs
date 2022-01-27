use crate::Parse;
use grape_ast::{InterfaceTypeDefinition, StringValue};
use grape_diagnostics::Message;
use grape_symbol::INTERFACE;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn interface_type_definition(
        &mut self,
    ) -> Result<Option<InterfaceTypeDefinition>, Message> {
        let description = self.string_value().ok();

        self.interface_type_definition_with_description(&description)
    }

    pub fn interface_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<InterfaceTypeDefinition>, Message> {
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
            let (end_span, fields) = if let Some((end_span, fields)) = self.field_definitions()? {
                (end_span, fields)
            } else if let Some(directive) = directives.last() {
                (directive.span, vec![])
            } else if let Some(interface) = implement_interfaces.last() {
                (interface.span, vec![])
            } else {
                (name.span, vec![])
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
