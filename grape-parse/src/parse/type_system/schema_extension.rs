use crate::{error, Parse};
use grape_ast::SchemaExtension;
use grape_diagnostics::Message;
use grape_span::Span;
use grape_symbol::{EXTEND, SCHEMA};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn schema_extension(&mut self) -> Result<Option<SchemaExtension>, Message> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            self.schema_extension_with_extend(&start_span)
        } else {
            Ok(None)
        }
    }

    pub fn schema_extension_with_extend(
        &mut self,
        start_span: &Span,
    ) -> Result<Option<SchemaExtension>, Message> {
        if self.current_token() == &TokenKind::Name(SCHEMA) {
            self.bump();

            let directives = self.directives()?;

            if let Some((end_span, fields)) = self.root_operation_type_definitions()? {
                Ok(Some(SchemaExtension {
                    span: start_span.with_end(&end_span),
                    directives,
                    fields,
                }))
            } else if let Some(directive) = directives.last() {
                Ok(Some(SchemaExtension {
                    span: start_span.with_end(&directive.span),
                    directives,
                    fields: vec![],
                }))
            } else {
                error!()
            }
        } else {
            Ok(None)
        }
    }
}
