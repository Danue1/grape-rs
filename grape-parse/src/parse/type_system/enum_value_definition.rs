use crate::{Error, Parse};
use grape_ast::EnumValueDefinition;
use grape_span::Span;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn enum_value_definitions(
        &mut self,
    ) -> Result<Option<(Span, Vec<EnumValueDefinition>)>, Error> {
        let start_span = if let (start_span, TokenKind::LeftBrace) = self.current() {
            let start_span = *start_span;

            self.bump();

            start_span
        } else {
            return Ok(None);
        };

        let mut values = vec![];

        while let Some(value) = self.enum_value_definition()? {
            values.push(value);
        }

        if values.is_empty() {
            return Err(Error::Unexpected);
        }

        if let (end_span, TokenKind::RightBrace) = self.current() {
            let span = start_span.with_end(end_span);

            self.bump();

            Ok(Some((span, values)))
        } else {
            Err(Error::Unexpected)
        }
    }

    pub fn enum_value_definition(&mut self) -> Result<Option<EnumValueDefinition>, Error> {
        let description = self.string_value().ok();

        if let Ok(name) = self.name() {
            let directives = self.directives()?;

            let start_span = if let Some(description) = &description {
                description.span
            } else {
                name.span
            };
            let end_span = if let Some(directive) = directives.last() {
                directive.span
            } else {
                name.span
            };

            Ok(Some(EnumValueDefinition {
                span: start_span.with_end(&end_span),
                description: description.clone(),
                name,
                directives,
            }))
        } else if description.is_some() {
            Err(Error::Unexpected)
        } else {
            Ok(None)
        }
    }
}
