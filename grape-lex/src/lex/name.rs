use crate::{expect, span_by, Lex};
use grape_diagnostics::Message;
use grape_token::{Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn name(&mut self) -> Result<Token, Message> {
        let (span, _) = span_by!(self => {
            let _ = expect!(self, '_' | 'a'..='z' | 'A'..='Z')?;
            while let Some(
                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9'
            ) = self.cursor.peek() {
                self.cursor.next();
            }
        });

        Ok(Token {
            span,
            kind: TokenKind::Name(self.intern(span)),
        })
    }
}
