use crate::{expect, span_by, Error, Lex};
use grape_span::Span;

impl<'lex> Lex<'lex> {
    pub fn comment(&mut self) -> Result<Span, Error> {
        let (span, _) = span_by!(self => {
            let _ = expect!(self, '#')?;
            self.cursor.line();
        });

        self.cursor.next();

        Ok(span)
    }
}
