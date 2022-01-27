mod comment;
mod name;
mod numeric;
mod string;

use crate::{
    expect, span_by, Cursor, CARRIAGE_RETUNR, COMMA, HORIZONTAL_TAB, NEW_LINE, SPACE, UNICODE_BOM,
};
use grape_diagnostics::{Diagnostics, Message};
use grape_span::Span;
use grape_symbol::{Symbol, SymbolInterner};
use grape_token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lex<'lex> {
    pub diagnostics: Diagnostics<'lex>,
    cursor: Cursor<'lex>,
    interner: SymbolInterner<'lex>,
}

impl<'lex> Lex<'lex> {
    pub fn new(source: &'lex str) -> Self {
        let mut lex = Lex {
            diagnostics: Diagnostics::new(source),
            cursor: Cursor::new(source),
            interner: SymbolInterner::new(),
        };
        lex.skip_whitespace();
        lex
    }

    fn intern(&mut self, span: Span) -> Symbol {
        let string = self.cursor.slice(span);

        self.interner.intern(string)
    }

    // [Spec](https://spec.graphql.org/October2021/#sec-Language.Source-Text.Ignored-Tokens)
    fn skip_whitespace(&mut self) {
        while matches!(
            self.cursor.peek(),
            Some(UNICODE_BOM | HORIZONTAL_TAB | SPACE | NEW_LINE | CARRIAGE_RETUNR | COMMA)
        ) {
            self.cursor.next();
        }
    }
}

impl<'lex> Iterator for Lex<'lex> {
    type Item = Result<Token, Message>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! punctuator {
            ($self:ident, $kind:ident) => {{
                let start = $self.cursor.index();
                $self.cursor.next();
                let end = $self.cursor.index();

                Some(Ok(Token {
                    span: Span::new(start, end),
                    kind: TokenKind::$kind,
                }))
            }};
        }

        loop {
            match self.cursor.peek() {
                // [Spec](https://spec.graphql.org/October2021/#sec-Language.Source-Text.Ignored-Tokens)
                Some(UNICODE_BOM | HORIZONTAL_TAB | SPACE | NEW_LINE | CARRIAGE_RETUNR | COMMA) => {
                    self.cursor.next();
                }
                Some('#') => {
                    self.cursor.next();
                    self.cursor.line();
                    self.cursor.next();
                }
                Some('!') => return punctuator!(self, Exclamation),
                Some('$') => return punctuator!(self, Dollar),
                Some('&') => return punctuator!(self, Ampersand),
                Some('(') => return punctuator!(self, LeftParens),
                Some(')') => return punctuator!(self, RightParens),
                Some('.') => {
                    let (span, _) = span_by!(self => {
                        self.cursor.next();
                        if let Err(error) = expect!(self, '.') {
                            return Some(Err(error));
                        }
                        if let Err(error) = expect!(self, '.') {
                            return Some(Err(error));
                        }
                    });

                    return Some(Ok(Token {
                        span,
                        kind: TokenKind::DotDotDot,
                    }));
                }
                Some(':') => return punctuator!(self, Colon),
                Some('=') => return punctuator!(self, Equal),
                Some('@') => return punctuator!(self, At),
                Some('[') => return punctuator!(self, LeftBracket),
                Some(']') => return punctuator!(self, RightBracket),
                Some('{') => return punctuator!(self, LeftBrace),
                Some('|') => return punctuator!(self, Pipe),
                Some('}') => return punctuator!(self, RightBrace),
                Some('"') => return Some(self.string()),
                Some('-' | '0'..='9') => return Some(self.numeric()),
                Some('_' | 'a'..='z' | 'A'..='Z') => return Some(self.name()),
                Some(_) => std::todo!(),
                None => return None,
            }
        }
    }
}
