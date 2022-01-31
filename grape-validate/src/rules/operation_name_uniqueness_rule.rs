use grape_ast::{DocumentContext, DocumentVisitor, OperationDefinition};
use grape_diagnostics::MessageBuilder;
use grape_symbol::Symbol;
use std::collections::HashSet;

/// [Spec](https://spec.graphql.org/draft/#sec-Operation-Name-Uniqueness)
#[derive(Default)]
pub struct OperationNameUniquenessRule {
    names: HashSet<Symbol>,
}

impl OperationNameUniquenessRule {
    pub fn new() -> Self {
        OperationNameUniquenessRule::default()
    }
}

impl<'rule, C: DocumentContext> DocumentVisitor<'rule, C> for OperationNameUniquenessRule {
    fn visit_operation_definition(
        &mut self,
        context: &C,
        operation_definition: &'rule OperationDefinition,
    ) {
        if let Some(name) = &operation_definition.name {
            if self.names.contains(&name.symbol) {
                let name = context.span(name.span);
                let message = MessageBuilder::error(format!(
                    "There can be only one operation named \"{}\".",
                    name
                ))
                .build();

                context.report(message);
            } else {
                self.names.insert(name.symbol);
            }
        }
    }
}
