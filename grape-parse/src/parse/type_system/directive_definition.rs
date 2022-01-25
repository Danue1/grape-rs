use crate::{expect, Error, Parse};
use grape_ast::{DirectiveDefinition, StringValue};
use grape_symbol::{DIRECTIVE, ON, REPEATABLE};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn directive_definition(&mut self) -> Result<Option<DirectiveDefinition>, Error> {
        let description = self.string_value().ok();

        self.directive_definition_with_description(&description)
    }

    pub fn directive_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<DirectiveDefinition>, Error> {
        if let (start_span, TokenKind::Name(DIRECTIVE)) = self.current() {
            let start_span = if let Some(description) = description {
                description.span
            } else {
                *start_span
            };

            self.bump();

            expect!(self, TokenKind::At);

            let name = self.name()?;
            let arguments = self.input_value_definitions_with_parens()?;
            let is_repeatable = self.current_token() == &TokenKind::Name(REPEATABLE);
            if is_repeatable {
                self.bump();
            }

            expect!(self, TokenKind::Name(ON));

            let locations = self.directive_locations()?;
            if let Some(location) = locations.last() {
                let end_span = location.span();
                let span = start_span.with_end(&end_span);

                Ok(Some(DirectiveDefinition {
                    span,
                    description: description.clone(),
                    name,
                    arguments,
                    is_repeatable,
                    locations,
                }))
            } else {
                Err(Error::Unexpected)
            }
        } else {
            Ok(None)
        }
    }
}
