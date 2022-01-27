#[macro_export]
macro_rules! expect {
    ($self:ident, $pat:pat_param $(| $tail_pat:pat_param)*) => {
        match $self.current_token() {
            $pat $(| $tail_pat)* => {
                $self.bump();
            },
            _ => $crate::error!()
        }
    };
}

#[macro_export]
macro_rules! spanned {
    ($definitions:ident) => {
        if let Some(last_definition) = $definitions.last() {
            let first_definition = unsafe { $definitions.get_unchecked(0) };

            first_definition.span().with_end(&last_definition.span())
        } else {
            grape_span::DUMMY_SPAN
        }
    };
}

#[macro_export]
macro_rules! error {
    () => {
        return Err(grape_diagnostics::MessageBuilder::error("unexpected token!").build())
    };
}
