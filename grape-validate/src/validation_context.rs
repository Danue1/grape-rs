#![warn(clippy::all)]

use grape_ast::{Definition, Document, DocumentContext, FragmentDefinition, Value};
use grape_diagnostics::{Diagnostics, Message};
use grape_span::Span;
use grape_symbol::Symbol;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValidationContext<'rule> {
    pub diagnostics: RefCell<Diagnostics<'rule>>,
    pub fragments: HashMap<Symbol, &'rule FragmentDefinition>,
    pub variables: &'rule HashMap<Symbol, Value>,
}

impl<'rule> ValidationContext<'rule> {
    pub fn new(
        source: &'rule str,
        document: &'rule Document,
        variables: &'rule HashMap<Symbol, Value>,
    ) -> Self {
        let diagnostics = RefCell::new(Diagnostics::new(source));
        let fragments = document
            .definitions
            .iter()
            .filter_map(|definition| match definition {
                Definition::Fragment(fragment) => Some((fragment.name.symbol, fragment)),
                _ => None,
            })
            .collect();

        ValidationContext {
            diagnostics,
            fragments,
            variables,
        }
    }
}

impl<'rule> DocumentContext for ValidationContext<'rule> {
    fn report(&self, message: Message) {
        self.diagnostics.borrow_mut().report(message);
    }

    fn span(&self, span: Span) -> &str {
        self.diagnostics.borrow().span(span)
    }

    fn fragment(&self, name: Symbol) -> Option<&FragmentDefinition> {
        match self.fragments.get(&name) {
            Some(fragment) => Some(fragment),
            None => None,
        }
    }

    fn variable(&self, name: Symbol) -> Option<&Value> {
        self.variables.get(&name)
    }
}
