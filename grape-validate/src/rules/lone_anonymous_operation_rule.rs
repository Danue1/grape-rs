use grape_ast::{
  Definition, Document, DocumentContext, DocumentVisitor, ExecutableDefinition, ExecutableDocument,
  OperationDefinition,
};
use grape_diagnostics::MessageBuilder;

/// [Spec](https://spec.graphql.org/draft/#sec-Lone-Anonymous-Operation)
pub struct LoneAnonymousOperationRule;

impl<'rule, C: DocumentContext> DocumentVisitor<'rule, C> for LoneAnonymousOperationRule {
  fn visit_document(&mut self, context: &C, document: &'rule Document) {
    let operation_count = document
      .definitions
      .iter()
      .filter(|definition| matches!(definition, Definition::Operation(_)))
      .count();

    if operation_count > 1 {
      for definition in document.definitions.iter() {
        self.visit_definition(context, definition);
      }
    }
  }

  fn visit_executable_document(
    &mut self,
    context: &C,
    executable_document: &'rule ExecutableDocument,
  ) {
    let operation_count = executable_document
      .definitions
      .iter()
      .filter(|definition| matches!(definition, ExecutableDefinition::Operation(_)))
      .count();

    if operation_count > 1 {
      for definition in executable_document.definitions.iter() {
        self.visit_executable_definition(context, definition);
      }
    }
  }

  fn visit_operation_definition(
    &mut self,
    context: &C,
    operation_definition: &'rule OperationDefinition,
  ) {
    if operation_definition.name.is_none() {
      let message =
        MessageBuilder::error("This anonymous operation must be the only defined operation.")
          .build();

      context.report(message);
    }
  }
}
