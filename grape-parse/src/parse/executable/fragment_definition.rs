use crate::{expect, Error, Parse};
use grape_ast::FragmentDefinition;
use grape_symbol::{FRAGMENT, ON};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn fragment_definition(&mut self) -> Result<Option<FragmentDefinition>, Error> {
        if let (start_span, TokenKind::Name(FRAGMENT)) = self.current() {
            let start_span = *start_span;

            self.bump();

            let name = self.name()?;

            expect!(self, TokenKind::Name(ON));

            let type_condition = self.name()?;
            let directives = self.directives()?;
            if let Some((end_span, selections)) = self.selections()? {
                Ok(Some(FragmentDefinition {
                    span: start_span.with_end(&end_span),
                    name,
                    type_condition,
                    directives,
                    selections,
                }))
            } else {
                Err(Error::Unexpected)
            }
        } else {
            Ok(None)
        }
    }
}
