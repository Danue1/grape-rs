use crate::Parse;
use grape_ast::{StringValue, TypeSystemDefinition};
use grape_diagnostics::Message;

impl<'parse> Parse<'parse> {
    pub fn type_system_definition(&mut self) -> Result<Option<TypeSystemDefinition>, Message> {
        let description = self.string_value().ok();

        self.type_system_definition_with_description(&description)
    }

    pub fn type_system_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<TypeSystemDefinition>, Message> {
        if let Some(definition) = self.schema_definition_with_description(description)? {
            Ok(Some(TypeSystemDefinition::Schema(definition)))
        } else if let Some(definition) = self.type_definition_with_description(description)? {
            Ok(Some(TypeSystemDefinition::Type(definition)))
        } else if let Some(definition) = self.directive_definition_with_description(description)? {
            Ok(Some(TypeSystemDefinition::Directive(definition)))
        } else {
            Ok(None)
        }
    }
}
