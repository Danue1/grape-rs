use crate::{Error, Parse};
use grape_ast::InputObjectTypeExtension;
use grape_span::Span;
use grape_symbol::{EXTEND, INPUT};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn input_object_type_extension(
        &mut self,
    ) -> Result<Option<InputObjectTypeExtension>, Error> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            self.input_object_type_extension_with_extend(&start_span)
        } else {
            Ok(None)
        }
    }

    pub fn input_object_type_extension_with_extend(
        &mut self,
        start_span: &Span,
    ) -> Result<Option<InputObjectTypeExtension>, Error> {
        if self.current_token() == &TokenKind::Name(INPUT) {
            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;
            let (end_span, values) =
                if let Some((end_span, values)) = self.input_value_definitions_with_brace()? {
                    (end_span, values)
                } else if let Some(directive) = directives.last() {
                    (directive.span, vec![])
                } else {
                    return Err(Error::Unexpected);
                };
            let span = start_span.with_end(&end_span);

            Ok(Some(InputObjectTypeExtension {
                span,
                name,
                directives,
                values,
            }))
        } else {
            Ok(None)
        }
    }
}
