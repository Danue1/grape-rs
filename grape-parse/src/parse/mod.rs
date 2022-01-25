mod entry;
mod executable;
mod language;
mod type_system;

use grape_lex::Lex;
use grape_span::{Span, DUMMY_SPAN};
use grape_token::{Token, TokenKind};
use std::iter::Peekable;

pub struct Parse<'parse> {
    lex: Peekable<Lex<'parse>>,
    span: Span,
}

impl<'parse> Parse<'parse> {
    pub fn new(source: &'parse str) -> Self {
        let mut parse = Parse {
            lex: Lex::new(source).peekable(),
            span: DUMMY_SPAN,
        };
        let (span, _) = parse.current();
        parse.span = *span;
        parse
    }

    #[inline]
    pub(crate) fn current(&mut self) -> (&Span, &TokenKind) {
        match self.lex.peek() {
            Some(Ok(Token { span, kind })) => (span, kind),
            _ => (&DUMMY_SPAN, &TokenKind::Eof),
        }
    }

    #[inline]
    pub(crate) fn current_token(&mut self) -> &TokenKind {
        match self.lex.peek() {
            Some(Ok(Token { span: _, kind })) => kind,
            _ => &TokenKind::Eof,
        }
    }

    fn bump(&mut self) {
        if let Some(Ok(Token { span, kind: _ })) = self.lex.next() {
            self.span = span;
        }
    }
}
