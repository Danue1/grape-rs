use crate::{expect, span_by, Lex};
use grape_diagnostics::Message;
use grape_span::Span;

impl<'lex> Lex<'lex> {
    pub fn comment(&mut self) -> Result<Span, Message> {
        let (span, _) = span_by!(self => {
            let _ = expect!(self, '#')?;
            self.cursor.line();
        });

        self.cursor.next();

        Ok(span)
    }
}
