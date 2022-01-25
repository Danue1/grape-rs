use crate::{Error, Parse};
use grape_ast::ScalarTypeExtension;
use grape_span::Span;
use grape_symbol::{EXTEND, SCALAR};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn scalar_type_extension(&mut self) -> Result<Option<ScalarTypeExtension>, Error> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            self.scalar_type_extension_with_extend(&start_span)
        } else {
            Ok(None)
        }
    }

    pub fn scalar_type_extension_with_extend(
        &mut self,
        start_span: &Span,
    ) -> Result<Option<ScalarTypeExtension>, Error> {
        if self.current_token() == &TokenKind::Name(SCALAR) {
            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;

            if let Some(directive) = directives.last() {
                let span = start_span.with_end(&directive.span);

                Ok(Some(ScalarTypeExtension {
                    span,
                    name,
                    directives,
                }))
            } else {
                Err(Error::Unexpected)
            }
        } else {
            Ok(None)
        }
    }
}
