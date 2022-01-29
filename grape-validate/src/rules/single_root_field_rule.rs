use crate::should_include;
use grape_ast::{
    walk_selection, Context, Definition, Document, ExecutableDefinition, ExecutableDocument, Field,
    FragmentSpread, InlineFragment, OperationDefinition, OperationType, Selection,
    TypeSystemDocument, TypeSystemExtensionDocument, Visitor,
};
use grape_diagnostics::MessageBuilder;
use grape_symbol::Symbol;
use std::collections::HashSet;

/// Subscription operations must have exactly one root field.
///
/// [Spec](https://spec.graphql.org/draft/#sec-Single-root-field)
#[derive(Default)]
pub struct SingleRootField {
    should_single_root_field_error: bool,
    visited_fragments: HashSet<Symbol>,
}

impl SingleRootField {
    pub fn new() -> Self {
        SingleRootField::default()
    }
}

impl<C: Context> Visitor<'_, C> for SingleRootField {
    fn visit_document(&mut self, context: &C, document: &Document) {
        for definition in &document.definitions {
            if let Definition::Operation(operation) = definition {
                self.visit_operation_definition(context, operation);
            }
        }
    }

    fn visit_type_system_document(
        &mut self,
        _context: &C,
        _type_system_document: &TypeSystemDocument,
    ) {
        //
    }

    fn visit_type_system_extension_document(
        &mut self,
        _context: &C,
        _type_system_extension_document: &TypeSystemExtensionDocument,
    ) {
        //
    }

    fn visit_executable_document(&mut self, context: &C, executable_document: &ExecutableDocument) {
        for definition in &executable_document.definitions {
            if let ExecutableDefinition::Operation(operation) = definition {
                self.visit_operation_definition(context, operation);
            }
        }
    }

    fn visit_operation_definition(
        &mut self,
        context: &C,
        operation_definition: &OperationDefinition,
    ) {
        if operation_definition.operation == OperationType::Subscription {
            for selection in operation_definition.selections.iter() {
                self.visit_selection(context, selection);
            }
        }
    }

    fn visit_selection(&mut self, context: &C, selection: &Selection) {
        if should_include(context, selection) {
            walk_selection(self, context, selection);
        }
    }

    fn visit_field(&mut self, context: &C, field: &Field) {
        let name = context.span(field.name.span);
        if name.starts_with("__") {
            let name = context.span(field.name.span);
            let message = MessageBuilder::error(format!(
                "\"{}\" is not allowed in a single root field.",
                name
            ))
            .build();

            context.report(message);
            return;
        }

        if !self.should_single_root_field_error {
            self.should_single_root_field_error = true;
        } else {
            let message = MessageBuilder::error(format!(
                "Subscription operations must have exactly one root field.\n\
                Multiple fields with the name \"{}\" are not allowed in a single root field.",
                name,
            ))
            .build();

            context.report(message);
        }
    }

    fn visit_fragment_spread(&mut self, context: &C, fragment_spread: &FragmentSpread) {
        if !self
            .visited_fragments
            .contains(&fragment_spread.name.symbol)
        {
            if let Some(fragment) = context.fragment(fragment_spread.name.symbol) {
                self.visited_fragments.insert(fragment_spread.name.symbol);
                for selection in fragment.selections.iter() {
                    self.visit_selection(context, selection);
                }
            }
        }
    }

    fn visit_inline_fragment(&mut self, context: &C, inline_fragment: &InlineFragment) {
        for selection in inline_fragment.selections.iter() {
            self.visit_selection(context, selection);
        }
    }
}
