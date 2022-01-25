use crate::{expect, Error, Parse};
use grape_ast::{InputValueDefinition, StringValue};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn input_value_definitions_with_parens(
        &mut self,
    ) -> Result<Vec<InputValueDefinition>, Error> {
        if self.current_token() == &TokenKind::LeftParens {
            self.bump();
        } else {
            return Ok(vec![]);
        };

        let mut arguments = if let Some(argument) = self.input_value_definition()? {
            vec![argument]
        } else {
            return Err(Error::Unexpected);
        };

        while let Some(argument) = self.input_value_definition()? {
            arguments.push(argument);
        }

        if self.current_token() == &TokenKind::RightParens {
            self.bump();

            Ok(arguments)
        } else {
            Err(Error::Unexpected)
        }
    }

    pub fn input_value_definitions_with_brace(
        &mut self,
    ) -> Result<Vec<InputValueDefinition>, Error> {
        if self.current_token() == &TokenKind::LeftBrace {
            self.bump();
        } else {
            return Ok(vec![]);
        };

        let mut arguments = if let Some(argument) = self.input_value_definition()? {
            vec![argument]
        } else {
            return Err(Error::Unexpected);
        };

        while let Some(argument) = self.input_value_definition()? {
            arguments.push(argument);
        }

        if self.current_token() == &TokenKind::RightBrace {
            self.bump();

            Ok(arguments)
        } else {
            Err(Error::Unexpected)
        }
    }

    pub fn input_value_definition(&mut self) -> Result<Option<InputValueDefinition>, Error> {
        let description = self.string_value().ok();

        self.input_value_definition_with_description(&description)
    }

    pub fn input_value_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<InputValueDefinition>, Error> {
        let name = if let Ok(name) = self.name() {
            name
        } else if description.is_some() {
            return Err(Error::Unexpected);
        } else {
            return Ok(None);
        };

        let start_span = if let Some(description) = &description {
            description.span
        } else {
            name.span
        };

        expect!(self, TokenKind::Colon);

        let ty = self.ty()?;
        let default_value = self.default_value()?;
        let directives = self.directives()?;

        let end_span = if let Some(directive) = directives.last() {
            directive.span
        } else if let Some(default_value) = &default_value {
            default_value.span()
        } else {
            ty.span
        };
        let span = start_span.with_end(&end_span);

        Ok(Some(InputValueDefinition {
            span,
            description: description.clone(),
            name,
            ty,
            default_value,
            directives,
        }))
    }
}
