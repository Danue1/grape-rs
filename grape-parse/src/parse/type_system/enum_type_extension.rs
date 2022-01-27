use crate::{error, Parse};
use grape_ast::EnumTypeExtension;
use grape_diagnostics::Message;
use grape_span::Span;
use grape_symbol::{ENUM, EXTEND};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn enum_type_extension(&mut self) -> Result<Option<EnumTypeExtension>, Message> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            self.enum_type_extension_with_extend(&start_span)
        } else {
            Ok(None)
        }
    }

    pub fn enum_type_extension_with_extend(
        &mut self,
        start_span: &Span,
    ) -> Result<Option<EnumTypeExtension>, Message> {
        if self.current_token() == &TokenKind::Name(ENUM) {
            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;
            let (end_span, values) =
                if let Some((end_span, values)) = self.enum_value_definitions()? {
                    (end_span, values)
                } else if let Some(directive) = directives.last() {
                    (directive.span, vec![])
                } else {
                    error!();
                };
            let span = start_span.with_end(&end_span);

            Ok(Some(EnumTypeExtension {
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
