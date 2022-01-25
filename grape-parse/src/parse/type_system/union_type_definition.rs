use crate::{Error, Parse};
use grape_ast::{Name, StringValue, UnionTypeDefinition};
use grape_symbol::UNION;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn union_type_definition(&mut self) -> Result<Option<UnionTypeDefinition>, Error> {
        let description = self.string_value().ok();

        self.union_type_definition_with_description(&description)
    }

    pub fn union_type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<UnionTypeDefinition>, Error> {
        if let (start_span, TokenKind::Name(UNION)) = self.current() {
            let start_span = if let Some(description) = description {
                description.span
            } else {
                *start_span
            };

            self.bump();

            let name = self.name()?;
            let directives = self.directives()?;
            let members = self.union_members()?;

            let end_span = if let Some(member) = members.last() {
                member.span
            } else if let Some(directive) = directives.last() {
                directive.span
            } else {
                name.span
            };
            let span = start_span.with_end(&end_span);

            Ok(Some(UnionTypeDefinition {
                span,
                description: description.clone(),
                name,
                directives,
                members,
            }))
        } else {
            Ok(None)
        }
    }

    fn union_members(&mut self) -> Result<Vec<Name>, Error> {
        if self.current_token() == &TokenKind::Equal {
            self.bump();
        } else {
            return Ok(vec![]);
        }

        if self.current_token() == &TokenKind::Pipe {
            self.bump();
        }

        let mut members = vec![self.name()?];

        while self.current_token() == &TokenKind::Pipe {
            self.bump();

            members.push(self.name()?);
        }

        Ok(members)
    }
}
