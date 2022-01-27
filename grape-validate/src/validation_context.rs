#![warn(clippy::all)]

use grape_ast::Context;
use grape_diagnostics::{Diagnostics, Message};
use grape_span::Span;

#[derive(Debug)]
pub struct ValidationContext<'rule> {
    pub diagnostics: Diagnostics<'rule>,
}

impl<'rule> ValidationContext<'rule> {
    pub fn new(source: &'rule str) -> Self {
        ValidationContext {
            diagnostics: Diagnostics::new(source),
        }
    }
}

impl<'rule> Context for ValidationContext<'rule> {
    fn report(&mut self, message: Message) {
        self.diagnostics.report(message);
    }

    fn span(&self, span: &Span) -> &str {
        self.diagnostics.span(span)
    }
}
