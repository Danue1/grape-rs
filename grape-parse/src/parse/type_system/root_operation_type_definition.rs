use crate::{expect, Error, Parse};
use grape_ast::RootOperationTypeDefinition;
use grape_span::Span;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn root_operation_type_definitions(
        &mut self,
    ) -> Result<Option<(Span, Vec<RootOperationTypeDefinition>)>, Error> {
        if let (start_span, TokenKind::LeftBrace) = self.current() {
            let start_span = *start_span;

            self.bump();

            if let Some(field) = self.root_operation_type_definition()? {
                let mut fields = vec![field];

                while let Some(field) = self.root_operation_type_definition()? {
                    fields.push(field);
                }

                if let (end_span, TokenKind::RightBrace) = self.current() {
                    let span = start_span.with_end(end_span);

                    self.bump();

                    Ok(Some((span, fields)))
                } else {
                    Err(Error::Unexpected)
                }
            } else {
                Err(Error::Unexpected)
            }
        } else {
            Ok(None)
        }
    }

    fn root_operation_type_definition(
        &mut self,
    ) -> Result<Option<RootOperationTypeDefinition>, Error> {
        if let Ok((start_span, key)) = self.operation_type() {
            expect!(self, TokenKind::Colon);

            let value = self.name()?;
            let span = start_span.with_end(&value.span);

            Ok(Some(RootOperationTypeDefinition { span, key, value }))
        } else {
            Ok(None)
        }
    }
}
