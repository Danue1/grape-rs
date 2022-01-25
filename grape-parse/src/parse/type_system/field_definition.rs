use crate::{expect, Error, Parse};
use grape_ast::{FieldDefinition, StringValue};
use grape_span::Span;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn field_definitions(&mut self) -> Result<Option<(Span, Vec<FieldDefinition>)>, Error> {
        if let (start_span, TokenKind::LeftBrace) = self.current() {
            let start_span = *start_span;

            self.bump();

            let mut fields = vec![];

            while let Some(field) = self.field_definition()? {
                fields.push(field);
            }

            if let (end_span, TokenKind::RightBrace) = self.current() {
                let span = start_span.with_end(end_span);

                self.bump();

                Ok(Some((span, fields)))
            } else {
                Err(Error::Unexpected)
            }
        } else {
            Ok(None)
        }
    }

    pub fn field_definition(&mut self) -> Result<Option<FieldDefinition>, Error> {
        let description = self.string_value().ok();

        self.field_definition_with_description(&description)
    }

    pub fn field_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<FieldDefinition>, Error> {
        let (start_span, name) = if let Ok(name) = self.name() {
            if let Some(description) = description {
                (description.span, name)
            } else {
                (name.span, name)
            }
        } else if description.is_some() {
            return Err(Error::Unexpected);
        } else {
            return Ok(None);
        };

        let arguments = self.input_value_definitions_with_parens()?;

        expect!(self, TokenKind::Colon);

        let ty = self.ty()?;
        let directives = self.directives()?;

        let end_span = if let Some(directive) = directives.last() {
            directive.span
        } else {
            ty.span
        };
        let span = start_span.with_end(&end_span);

        Ok(Some(FieldDefinition {
            span,
            description: description.clone(),
            name,
            arguments,
            ty,
            directives,
        }))
    }
}
