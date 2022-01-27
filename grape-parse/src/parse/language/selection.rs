use crate::{error, Parse};
use grape_ast::{Field, FragmentSpread, InlineFragment, Name, Selection};
use grape_diagnostics::Message;
use grape_span::Span;
use grape_symbol::ON;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn selections(&mut self) -> Result<Option<(Span, Vec<Selection>)>, Message> {
        if let (start_span, TokenKind::LeftBrace) = self.current() {
            let start_span = *start_span;

            self.bump();

            let mut selections = vec![];

            while let Some(selection) = self.selection()? {
                selections.push(selection);
            }

            if let (end_span, TokenKind::RightBrace) = self.current() {
                let span = start_span.with_end(end_span);

                self.bump();

                Ok(Some((span, selections)))
            } else {
                error!()
            }
        } else {
            Ok(None)
        }
    }

    fn selection(&mut self) -> Result<Option<Selection>, Message> {
        match self.current() {
            (start_span, TokenKind::DotDotDot) => {
                let start_span = *start_span;

                self.bump();

                match self.current() {
                    (_, TokenKind::Name(ON)) => {
                        self.bump();

                        let name = self.name()?;
                        let directives = self.directives()?;
                        let (end_span, selections) =
                            if let Some((span, selections)) = self.selections()? {
                                (span, selections)
                            } else {
                                error!();
                            };

                        Ok(Some(Selection::InlineFragment(InlineFragment {
                            span: start_span.with_end(&end_span),
                            name,
                            directives,
                            selections,
                        })))
                    }
                    (end_span, TokenKind::Name(name)) => {
                        let end_span = *end_span;
                        let name = Name {
                            span: end_span,
                            symbol: *name,
                        };

                        self.bump();

                        let directives = self.directives()?;
                        let end_span = if let Some(directive) = directives.last() {
                            directive.span
                        } else {
                            end_span
                        };

                        Ok(Some(Selection::FragmentSpread(FragmentSpread {
                            span: start_span.with_end(&end_span),
                            name,
                            directives,
                        })))
                    }
                    _ => error!(),
                }
            }
            (span, TokenKind::Name(symbol)) => {
                let start_span = *span;
                let name_or_alias = Name {
                    span: *span,
                    symbol: *symbol,
                };

                self.bump();

                let (alias, name) = if self.current_token() == &TokenKind::Colon {
                    self.bump();

                    if let (name_span, TokenKind::Name(name)) = self.current() {
                        let name = Name {
                            span: *name_span,
                            symbol: *name,
                        };

                        self.bump();

                        (Some(name_or_alias), name)
                    } else {
                        error!();
                    }
                } else {
                    (None, name_or_alias)
                };

                let arguments = if let Some((_, arguments)) = self.arguments()? {
                    arguments
                } else {
                    vec![]
                };
                let directives = self.directives()?;
                let (end_span, selections) = if let Some((span, selections)) = self.selections()? {
                    (span, selections)
                } else if let Some(directive) = directives.last() {
                    (directive.span, vec![])
                } else {
                    (name.span, vec![])
                };

                Ok(Some(Selection::Field(Field {
                    span: start_span.with_end(&end_span),
                    alias,
                    name,
                    arguments,
                    directives,
                    selections,
                })))
            }
            _ => Ok(None),
        }
    }
}
