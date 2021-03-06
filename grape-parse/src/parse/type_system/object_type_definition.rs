use crate::Parse;
use grape_ast::{Name, ObjectTypeDefinition, StringValue};
use grape_diagnostics::Message;
use grape_symbol::{IMPLEMENTS, TYPE};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn object_type_definition(&mut self) -> Result<Option<ObjectTypeDefinition>, Message> {
        let description = self.string_value().ok();

        self.object_type_definition_with_description(&description)
    }

    pub fn object_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<ObjectTypeDefinition>, Message> {
        if let (start_span, TokenKind::Name(TYPE)) = self.current() {
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

            Ok(Some(ObjectTypeDefinition {
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

    pub(crate) fn implement_interfaces(&mut self) -> Result<Vec<Name>, Message> {
        if self.current_token() == &TokenKind::Name(IMPLEMENTS) {
            self.bump();
        } else {
            return Ok(vec![]);
        }

        if self.current_token() == &TokenKind::Ampersand {
            self.bump();
        }

        let mut implement_interfaces = vec![self.name()?];

        while self.current_token() == &TokenKind::Ampersand {
            self.bump();

            implement_interfaces.push(self.name()?);
        }

        Ok(implement_interfaces)
    }
}
