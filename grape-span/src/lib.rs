#![warn(clippy::all)]

pub const DUMMY_SPAN: Span = Span { start: 0, end: 0 };

#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub const fn with_start(self, span: &Span) -> Self {
        Span {
            start: span.start,
            end: self.end,
        }
    }

    pub const fn with_end(self, span: &Span) -> Self {
        Span {
            start: self.start,
            end: span.end,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}
