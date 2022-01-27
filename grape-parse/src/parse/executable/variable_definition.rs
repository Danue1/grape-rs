use crate::{expect, Parse};
use grape_ast::VariableDefinition;
use grape_diagnostics::Message;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn variable_definitions(&mut self) -> Result<Vec<VariableDefinition>, Message> {
        if self.current_token() == &TokenKind::LeftParens {
            self.bump();
        } else {
            return Ok(vec![]);
        }

        let mut variables = vec![];

        while let Some(variable) = self.variable_definition()? {
            variables.push(variable);
        }

        expect!(self, TokenKind::RightParens);

        Ok(variables)
    }

    fn variable_definition(&mut self) -> Result<Option<VariableDefinition>, Message> {
        let start_span = self.span;
        if self.current_token() == &TokenKind::Dollar {
            self.bump();
        } else {
            return Ok(None);
        };

        let name = self.name()?;
        expect!(self, TokenKind::Colon);
        let ty = self.ty()?;
        let default_value = self.default_value()?;
        let directives = self.directives()?;
        let span = start_span.with_end(&self.span);

        Ok(Some(VariableDefinition {
            span,
            name,
            ty,
            default_value,
            directives,
        }))
    }
}
