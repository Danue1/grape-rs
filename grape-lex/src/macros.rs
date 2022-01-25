#[macro_export]
macro_rules! expect {
    ($self:ident, $expr:pat_param $( | $tail_expr:pat_param)*) => {{
        match $self.cursor.peek() {
            Some($expr $(| $tail_expr)*) => {
                $self.cursor.next();
                Ok(())
            },
            _ => Err(Error::Unexpected),
        }
    }};
}

#[macro_export]
macro_rules! unicode_char {
    ($self:ident) => {
        if !matches!($self.cursor.peek(), Some('0'..='9' | 'a'..='z' | 'A'..='Z')) {
            return Err(Error::Unexpected);
        }
    };
}

#[macro_export]
macro_rules! span_by {
    ($self:ident => { $($tt:tt)* }) => {{
        let start = $self.cursor.index();
        let ret = { $($tt)* };
        let end = $self.cursor.index();

        (grape_span::Span { start, end }, ret)
    }};
}
