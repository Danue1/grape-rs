use crate::{
    expect, span_by, unicode_char, Lex, CARRIAGE_RETUNR, HORIZONTAL_TAB, NEW_LINE, SPACE,
    UNICODE_END,
};
use grape_diagnostics::{Message, MessageBuilder};
use grape_span::Span;
use grape_symbol::EMPTY_STRING;
use grape_token::{Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn string(&mut self) -> Result<Token, Message> {
        let (span, is_block) = span_by!(self => {
            let start = self.cursor.index();
            let _ = expect!(self, '"')?;

            if matches!(self.cursor.peek(), Some('"')) {
                self.cursor.next();

                let end = self.cursor.index();

                if !matches!(self.cursor.peek(), Some('"')) {
                    return Ok(Token { span: Span::new(start, end), kind: TokenKind::String(EMPTY_STRING, false)});
                }
                self.cursor.next();

                // Block String Mode
                loop {
                    match self.cursor.peek() {
                        Some('"') => {
                            self.cursor.next();
                            if matches!(self.cursor.peek(), Some('"')) {
                                self.cursor.next();
                                if matches!(self.cursor.peek(), Some('"')) {
                                    self.cursor.next();
                                    break;
                                }
                            }
                        }
                        Some(
                            HORIZONTAL_TAB
                            | NEW_LINE
                            | CARRIAGE_RETUNR
                            | SPACE..=UNICODE_END
                        ) => {
                            self.cursor.next();
                        }
                        _ => return Err(MessageBuilder::error("unexpected character").build()),
                    }
                }

                true
            } else {
                // Singleline String Mode
                loop {
                    match self.cursor.peek() {
                        Some('"') => {
                            self.cursor.next();
                            break;
                        }
                        Some('\\') => {
                            self.cursor.next();
                            match self.cursor.peek() {
                                Some('u') => {
                                    self.cursor.next();
                                    unicode_char!(self);
                                    unicode_char!(self);
                                    unicode_char!(self);
                                    unicode_char!(self);
                                }
                                Some(
                                    '\\'
                                    | '/'
                                    | 'b'
                                    | 'f'
                                    | 'n'
                                    | 'r'
                                    | 't'
                                ) => {
                                    self.cursor.next();
                                }
                                _ => return Err(MessageBuilder::error("unexpected character").build())
                            }
                        }
                        Some(
                            HORIZONTAL_TAB
                            | NEW_LINE
                            | CARRIAGE_RETUNR
                            | SPACE..=UNICODE_END
                        ) => {
                            self.cursor.next();
                        }
                        _ => return Err(MessageBuilder::error("unexpected character").build())
                    }
                }

                false
            }
        });

        Ok(Token {
            span,
            kind: TokenKind::String(self.intern(span), is_block),
        })
    }
}
