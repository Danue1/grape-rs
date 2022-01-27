use crate::{error, Parse};
use grape_ast::InterfaceTypeExtension;
use grape_diagnostics::Message;
use grape_span::Span;
use grape_symbol::{EXTEND, INTERFACE};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn interface_type_extension(&mut self) -> Result<Option<InterfaceTypeExtension>, Message> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            self.interface_type_extension_with_extend(&start_span)
        } else {
            Ok(None)
        }
    }

    pub fn interface_type_extension_with_extend(
        &mut self,
        start_span: &Span,
    ) -> Result<Option<InterfaceTypeExtension>, Message> {
        if self.current_token() == &TokenKind::Name(INTERFACE) {
            self.bump();

            let name = self.name()?;
            let implement_interfaces = self.implement_interfaces()?;
            let directives = self.directives()?;
            let (end_span, fields) = if let Some((end_span, fields)) = self.field_definitions()? {
                (end_span, fields)
            } else if let Some(directive) = directives.last() {
                (directive.span, vec![])
            } else if let Some(interface) = implement_interfaces.last() {
                (interface.span, vec![])
            } else {
                error!()
            };

            let span = start_span.with_end(&end_span);

            Ok(Some(InterfaceTypeExtension {
                span,
                name,
                implement_interfaces,
                directives,
                fields,
            }))
        } else {
            Ok(None)
        }
    }
}
