use crate::Parse;
use grape_ast::{StringValue, TypeDefinition};
use grape_diagnostics::Message;

impl<'parse> Parse<'parse> {
    pub fn type_definition(&mut self) -> Result<Option<TypeDefinition>, Message> {
        let description = self.string_value().ok();

        self.type_definition_with_description(&description)
    }

    pub fn type_definition_with_description(
        &mut self,
        description: &Option<StringValue>,
    ) -> Result<Option<TypeDefinition>, Message> {
        if let Some(definition) = self.scalar_type_definition_with_description(description)? {
            Ok(Some(TypeDefinition::Scalar(definition)))
        } else if let Some(definition) =
            self.object_type_definition_with_description(description)?
        {
            Ok(Some(TypeDefinition::Object(definition)))
        } else if let Some(definition) =
            self.interface_type_definition_with_description(description)?
        {
            Ok(Some(TypeDefinition::Interface(definition)))
        } else if let Some(definition) = self.union_type_definition_with_description(description)? {
            Ok(Some(TypeDefinition::Union(definition)))
        } else if let Some(definition) = self.enum_type_definition_with_description(description)? {
            Ok(Some(TypeDefinition::Enum(definition)))
        } else if let Some(definition) =
            self.input_object_type_definition_with_description(description)?
        {
            Ok(Some(TypeDefinition::InputObject(definition)))
        } else {
            Ok(None)
        }
    }
}
