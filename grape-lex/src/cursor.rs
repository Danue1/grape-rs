use grape_span::Span;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub(crate) struct Cursor<'lex> {
    source: &'lex str,
    iter: Peekable<Chars<'lex>>,
    index: usize,
}

impl<'lex> Cursor<'lex> {
    pub(crate) fn new(source: &'lex str) -> Self {
        Cursor {
            source,
            iter: source.chars().peekable(),
            index: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    #[inline(always)]
    pub(crate) fn line(&mut self) -> Option<&'lex str> {
        match self.source[self.index..].lines().next() {
            Some(line) => {
                let count = line.len();
                for _ in 0..count {
                    self.next();
                }

                Some(line)
            }
            None => None,
        }
    }

    #[inline(always)]
    pub(crate) fn peek(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }

    #[inline(always)]
    pub(crate) fn slice(&mut self, span: Span) -> &'lex str {
        &self.source[span.start..span.end]
    }
}

impl<'lex> Iterator for Cursor<'lex> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.iter.next()
    }
}
