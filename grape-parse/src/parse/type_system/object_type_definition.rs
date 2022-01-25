use crate::{Error, Parse};
use grape_ast::{Name, ObjectTypeDefinition, StringValue};
use grape_symbol::{IMPLEMENTS, TYPE};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn object_type_definition(&mut self) -> Result<Option<ObjectTypeDefinition>, Error> {
        let description = self.string_value().ok();

        self.object_type_definition_with_description(&description)
    }

    pub fn object_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<ObjectTypeDefinition>, Error> {
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

    pub(crate) fn implement_interfaces(&mut self) -> Result<Vec<Name>, Error> {
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
