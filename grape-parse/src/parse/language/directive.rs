use crate::{Error, Parse};
use grape_ast::Directive;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn directives(&mut self) -> Result<Vec<Directive>, Error> {
        let mut variables = vec![];

        while let Some(variable) = self.directive()? {
            variables.push(variable);
        }

        Ok(variables)
    }

    fn directive(&mut self) -> Result<Option<Directive>, Error> {
        if let (start_span, TokenKind::At) = self.current() {
            let start_span = *start_span;

            self.bump();

            let name = self.name()?;
            let (end_span, arguments) = if let Some((span, arguments)) = self.arguments()? {
                (span, arguments)
            } else {
                (name.span, vec![])
            };

            Ok(Some(Directive {
                span: start_span.with_end(&end_span),
                name,
                arguments,
            }))
        } else {
            Ok(None)
        }
    }
}
