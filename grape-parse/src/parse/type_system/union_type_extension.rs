use crate::{Error, Parse};
use grape_ast::UnionTypeExtension;
use grape_span::Span;
use grape_symbol::{EXTEND, UNION};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn union_type_extension(&mut self) -> Result<Option<UnionTypeExtension>, Error> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            self.union_type_extension_with_extend(&start_span)
        } else {
            Ok(None)
        }
    }

    pub fn union_type_extension_with_extend(
        &mut self,
        start_span: &Span,
    ) -> Result<Option<UnionTypeExtension>, Error> {
        if self.current_token() == &TokenKind::Name(UNION) {
            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;
            let members = self.union_members()?;

            let end_span = if let Some(member) = members.last() {
                member.span
            } else if let Some(directive) = directives.last() {
                directive.span
            } else {
                return Err(Error::Unexpected);
            };
            let span = start_span.with_end(&end_span);

            Ok(Some(UnionTypeExtension {
                span,
                name,
                directives,
                members,
            }))
        } else {
            Ok(None)
        }
    }
}
