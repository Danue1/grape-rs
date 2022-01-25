use crate::{expect, span_by, Error, Lex};
use grape_token::{Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn numeric(&mut self) -> Result<Token, Error> {
        let (integer_span, _) = span_by!(self => {
            if matches!(self.cursor.peek(), Some('-')) {
                self.cursor.next();
            };
            match self.cursor.next() {
                Some('0') => {}
                Some('1'..='9') => {
                    while matches!(self.cursor.peek(), Some('0'..='9')) {
                        self.cursor.next();
                    }
                }
                _ => return Err(Error::Unexpected),
            }
        });
        if !matches!(self.cursor.peek(), Some('.' | 'e' | 'E')) {
            let symbol = self.intern(integer_span.with_end(&integer_span));

            return Ok(Token {
                span: integer_span,
                kind: TokenKind::Integer(symbol),
            });
        }

        let (float_span, _) = span_by!(self => {
            if matches!(self.cursor.peek(), Some('.')) {
                self.cursor.next();
                while matches!(self.cursor.peek(), Some('0'..='9')) {
                    self.cursor.next();
                }
            }

            if matches!(self.cursor.peek(), Some('e' | 'E')) {
                self.cursor.next();
                if matches!(self.cursor.peek(), Some('+' | '-')) {
                    self.cursor.next();
                }
                let _ = expect!(self, '0'..='9')?;
                while matches!(self.cursor.peek(), Some('0'..='9')) {
                    self.cursor.next();
                }
            }
        });

        if float_span.is_empty() {
            Err(Error::Unexpected)
        } else {
            let span = integer_span.with_end(&float_span);
            let symbol = self.intern(span);

            Ok(Token {
                span,
                kind: TokenKind::Float(symbol),
            })
        }
    }
}
