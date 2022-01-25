use crate::{
    Argument, BooleanValue, Definition, Directive, DirectiveDefinition, DirectiveLocation,
    Document, EnumTypeDefinition, EnumTypeExtension, EnumValue, EnumValueDefinition,
    ExecutableDefinition, ExecutableDirectiveLocation, ExecutableDirectiveLocationKind,
    ExecutableDocument, Field, FieldDefinition, FloatValue, FragmentDefinition, FragmentSpread,
    InlineFragment, InputObjectTypeDefinition, InputObjectTypeExtension, InputValueDefinition,
    IntValue, InterfaceTypeDefinition, InterfaceTypeExtension, ListValue, Name, NullValue,
    ObjectField, ObjectTypeDefinition, ObjectTypeExtension, ObjectValue, OperationDefinition,
    OperationType, RootOperationTypeDefinition, ScalarTypeDefinition, ScalarTypeExtension,
    SchemaDefinition, SchemaExtension, Selection, StringValue, Type, TypeDefinition, TypeExtension,
    TypeKind, TypeSystemDefinition, TypeSystemDefinitionOrExtension, TypeSystemDirectiveLocation,
    TypeSystemDirectiveLocationKind, TypeSystemDocument, TypeSystemExtension,
    TypeSystemExtensionDocument, UnionTypeDefinition, UnionTypeExtension, Value, Variable,
    VariableDefinition,
};

#[allow(unused_variables)]
pub trait Visitor<'ast>: Sized {
    fn visit_argument(&mut self, argument: &'ast Argument) {
        walk_argument(self, argument);
    }
    fn visit_boolean_value(&mut self, boolean_value: &'ast BooleanValue) {
        //
    }
    fn visit_definition(&mut self, definition: &'ast Definition) {
        walk_definition(self, definition);
    }
    fn visit_directive(&mut self, directive: &'ast Directive) {
        walk_directive(self, directive);
    }
    fn visit_directive_definition(&mut self, directive_definition: &'ast DirectiveDefinition) {
        walk_directive_definition(self, directive_definition);
    }
    fn visit_directive_location(&mut self, directive_location: &'ast DirectiveLocation) {
        walk_directive_location(self, directive_location);
    }
    fn visit_document(&mut self, document: &'ast Document) {
        walk_document(self, document);
    }
    fn visit_enum_type_definition(&mut self, enum_type_definition: &'ast EnumTypeDefinition) {
        walk_enum_type_definition(self, enum_type_definition);
    }
    fn visit_enum_type_extension(&mut self, enum_type_extension: &'ast EnumTypeExtension) {
        walk_enum_type_extension(self, enum_type_extension);
    }
    fn visit_enum_value(&mut self, enum_value: &'ast EnumValue) {
        walk_enum_value(self, enum_value);
    }
    fn visit_enum_value_definition(&mut self, enum_value_definition: &'ast EnumValueDefinition) {
        walk_enum_value_definition(self, enum_value_definition);
    }
    fn visit_executable_definition(&mut self, executable_definition: &'ast ExecutableDefinition) {
        walk_executable_definition(self, executable_definition);
    }
    fn visit_executable_directive_location(
        &mut self,
        executable_directive_location: &'ast ExecutableDirectiveLocation,
    ) {
        walk_executable_directive_location(self, executable_directive_location);
    }
    fn visit_executable_directive_location_kind(
        &mut self,
        executable_directive_location_kind: &'ast ExecutableDirectiveLocationKind,
    ) {
        //
    }
    fn visit_executable_document(&mut self, executable_document: &'ast ExecutableDocument) {
        walk_executable_document(self, executable_document);
    }
    fn visit_field(&mut self, field: &'ast Field) {
        walk_field(self, field);
    }
    fn visit_field_definition(&mut self, field_definition: &'ast FieldDefinition) {
        walk_field_definition(self, field_definition);
    }
    fn visit_float_value(&mut self, float_value: &'ast FloatValue) {
        //
    }
    fn visit_fragment_definition(&mut self, fragment_definition: &'ast FragmentDefinition) {
        walk_fragment_definition(self, fragment_definition);
    }
    fn visit_fragment_spread(&mut self, fragment_spread: &'ast FragmentSpread) {
        walk_fragment_spread(self, fragment_spread);
    }
    fn visit_inline_fragment(&mut self, inline_fragment: &'ast InlineFragment) {
        walk_inline_fragment(self, inline_fragment);
    }
    fn visit_input_object_type_definition(
        &mut self,
        input_object_type_definition: &'ast InputObjectTypeDefinition,
    ) {
        walk_input_object_type_definition(self, input_object_type_definition);
    }
    fn visit_input_object_type_extension(
        &mut self,
        input_object_type_extension: &'ast InputObjectTypeExtension,
    ) {
        walk_input_object_type_extension(self, input_object_type_extension);
    }
    fn visit_input_value_definition(&mut self, input_value_definition: &'ast InputValueDefinition) {
        walk_input_value_definition(self, input_value_definition);
    }
    fn visit_int_value(&mut self, int_value: &'ast IntValue) {
        //
    }
    fn visit_interface_type_definition(
        &mut self,
        interface_type_definition: &'ast InterfaceTypeDefinition,
    ) {
        walk_interface_type_definition(self, interface_type_definition);
    }
    fn visit_interface_type_extension(
        &mut self,
        interface_type_extension: &'ast InterfaceTypeExtension,
    ) {
        walk_interface_type_extension(self, interface_type_extension);
    }
    fn visit_list_value(&mut self, list_value: &'ast ListValue) {
        walk_list_value(self, list_value);
    }
    fn visit_name(&mut self, name: &'ast Name) {
        //
    }
    fn visit_null_value(&mut self, null_value: &'ast NullValue) {
        //
    }
    fn visit_object_field(&mut self, object_field: &'ast ObjectField) {
        walk_object_field(self, object_field);
    }
    fn visit_object_type_definition(&mut self, object_type_definition: &'ast ObjectTypeDefinition) {
        walk_object_type_definition(self, object_type_definition);
    }
    fn visit_object_type_extension(&mut self, object_type_extension: &'ast ObjectTypeExtension) {
        walk_object_type_extension(self, object_type_extension);
    }
    fn visit_object_value(&mut self, object_value: &'ast ObjectValue) {
        walk_object_value(self, object_value);
    }
    fn visit_operation_definiion(&mut self, operation_definiion: &'ast OperationDefinition) {
        walk_operation_definiion(self, operation_definiion);
    }
    fn visit_operation_type(&mut self, operation_type: &'ast OperationType) {
        //
    }
    fn visit_root_operation_type_definition(
        &mut self,
        root_operation_type_definition: &'ast RootOperationTypeDefinition,
    ) {
        walk_root_operation_type_definition(self, root_operation_type_definition);
    }
    fn visit_scalar_type_definition(&mut self, scalar_type_definition: &'ast ScalarTypeDefinition) {
        walk_scalar_type_definition(self, scalar_type_definition);
    }
    fn visit_scalar_type_extension(&mut self, scalar_type_extension: &'ast ScalarTypeExtension) {
        walk_scalar_type_extension(self, scalar_type_extension);
    }
    fn visit_schema_definition(&mut self, schema_definition: &'ast SchemaDefinition) {
        walk_schema_definition(self, schema_definition);
    }
    fn visit_schema_extension(&mut self, schema_extension: &'ast SchemaExtension) {
        walk_schema_extension(self, schema_extension);
    }
    fn visit_selection(&mut self, selection: &'ast Selection) {
        walk_selection(self, selection);
    }
    fn visit_string_value(&mut self, string_value: &'ast StringValue) {
        //
    }
    fn visit_ty(&mut self, ty: &'ast Type) {
        walk_ty(self, ty);
    }
    fn visit_type_definition(&mut self, type_definition: &'ast TypeDefinition) {
        walk_type_definition(self, type_definition);
    }
    fn visit_type_extension(&mut self, type_extension: &'ast TypeExtension) {
        walk_type_extension(self, type_extension);
    }
    fn visit_type_kind(&mut self, type_kind: &'ast TypeKind) {
        walk_type_kind(self, type_kind);
    }
    fn visit_type_system_definition(&mut self, type_system_definition: &'ast TypeSystemDefinition) {
        walk_type_system_definition(self, type_system_definition);
    }
    fn visit_type_system_definition_or_extension(
        &mut self,
        type_system_definition_or_extension: &'ast TypeSystemDefinitionOrExtension,
    ) {
        walk_type_system_definition_or_extension(self, type_system_definition_or_extension);
    }
    fn visit_type_system_directive_location(
        &mut self,
        type_system_directive_location: &'ast TypeSystemDirectiveLocation,
    ) {
        walk_type_system_directive_location(self, type_system_directive_location);
    }
    fn visit_type_system_directive_location_kind(
        &mut self,
        type_system_directive_location_kind: &'ast TypeSystemDirectiveLocationKind,
    ) {
        //
    }
    fn visit_type_system_document(&mut self, type_system_document: &'ast TypeSystemDocument) {
        walk_type_system_document(self, type_system_document);
    }
    fn visit_type_system_extension(&mut self, type_system_extension: &'ast TypeSystemExtension) {
        walk_type_system_extension(self, type_system_extension);
    }
    fn visit_type_system_extension_document(
        &mut self,
        type_system_extension_document: &'ast TypeSystemExtensionDocument,
    ) {
        walk_type_system_extension_document(self, type_system_extension_document);
    }
    fn visit_union_type_definition(&mut self, union_type_definition: &'ast UnionTypeDefinition) {
        walk_union_type_definition(self, union_type_definition);
    }
    fn visit_union_type_extension(&mut self, union_type_extension: &'ast UnionTypeExtension) {
        walk_union_type_extension(self, union_type_extension);
    }
    fn visit_value(&mut self, value: &'ast Value) {
        walk_value(self, value);
    }
    fn visit_variable(&mut self, variable: &'ast Variable) {
        walk_variable(self, variable);
    }
    fn visit_variable_definition(&mut self, variable_definition: &'ast VariableDefinition) {
        walk_variable_definition(self, variable_definition);
    }
}

pub fn walk_argument<'a, V: Visitor<'a>>(visitor: &mut V, argument: &'a Argument) {
    visitor.visit_name(&argument.name);
    visitor.visit_value(&argument.value);
}

pub fn walk_definition<'a, V: Visitor<'a>>(visitor: &mut V, definition: &'a Definition) {
    match definition {
        Definition::Operation(ref definition) => visitor.visit_operation_definiion(definition),
        Definition::Fragment(ref definition) => visitor.visit_fragment_definition(definition),
        Definition::TypeSystem(ref definition) => visitor.visit_type_system_definition(definition),
        Definition::Extension(ref definition) => visitor.visit_type_system_extension(definition),
    }
}

pub fn walk_directive<'a, V: Visitor<'a>>(visitor: &mut V, directive: &'a Directive) {
    visitor.visit_name(&directive.name);
    for argument in directive.arguments.iter() {
        visitor.visit_argument(argument);
    }
}

pub fn walk_directive_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    directive_definition: &'a DirectiveDefinition,
) {
    if let Some(ref description) = directive_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&directive_definition.name);
    for argument in directive_definition.arguments.iter() {
        visitor.visit_input_value_definition(argument);
    }
    for location in directive_definition.locations.iter() {
        visitor.visit_directive_location(location);
    }
}

pub fn walk_directive_location<'a, V: Visitor<'a>>(
    visitor: &mut V,
    directive_location: &'a DirectiveLocation,
) {
    match directive_location {
        DirectiveLocation::Executable(location) => {
            visitor.visit_executable_directive_location(location);
        }
        DirectiveLocation::TypeSystem(location) => {
            visitor.visit_type_system_directive_location(location);
        }
    }
}

pub fn walk_document<'a, V: Visitor<'a>>(visitor: &mut V, document: &'a Document) {
    for definition in document.definitions.iter() {
        visitor.visit_definition(definition);
    }
}

pub fn walk_enum_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    enum_type_definition: &'a EnumTypeDefinition,
) {
    if let Some(ref description) = enum_type_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&enum_type_definition.name);
    for directive in enum_type_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
    for value in enum_type_definition.values.iter() {
        visitor.visit_enum_value_definition(value);
    }
}

pub fn walk_enum_type_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    enum_type_extension: &'a EnumTypeExtension,
) {
    visitor.visit_name(&enum_type_extension.name);
    for directive in enum_type_extension.directives.iter() {
        visitor.visit_directive(directive);
    }
    for value in enum_type_extension.values.iter() {
        visitor.visit_enum_value_definition(value);
    }
}

pub fn walk_enum_value<'a, V: Visitor<'a>>(visitor: &mut V, enum_value: &'a EnumValue) {
    visitor.visit_name(&enum_value.name);
}

pub fn walk_enum_value_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    enum_value_definition: &'a EnumValueDefinition,
) {
    if let Some(ref description) = enum_value_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&enum_value_definition.name);
    for directive in enum_value_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
}

pub fn walk_executable_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    executable_definition: &'a ExecutableDefinition,
) {
    match executable_definition {
        ExecutableDefinition::Operation(definition) => {
            visitor.visit_operation_definiion(definition);
        }
        ExecutableDefinition::Fragment(definition) => visitor.visit_fragment_definition(definition),
    }
}

pub fn walk_executable_directive_location<'a, V: Visitor<'a>>(
    visitor: &mut V,
    executable_directive_location: &'a ExecutableDirectiveLocation,
) {
    visitor.visit_executable_directive_location_kind(&executable_directive_location.kind);
}

pub fn walk_executable_document<'a, V: Visitor<'a>>(
    visitor: &mut V,
    executable_document: &'a ExecutableDocument,
) {
    for definition in executable_document.definitions.iter() {
        visitor.visit_executable_definition(definition);
    }
}

pub fn walk_field<'a, V: Visitor<'a>>(visitor: &mut V, field: &'a Field) {
    if let Some(ref alias) = field.alias {
        visitor.visit_name(alias);
    }
    visitor.visit_name(&field.name);
    for argument in field.arguments.iter() {
        visitor.visit_argument(argument);
    }
    for directive in field.directives.iter() {
        visitor.visit_directive(directive);
    }
    for selection in field.selections.iter() {
        visitor.visit_selection(selection);
    }
}

pub fn walk_field_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    field_definition: &'a FieldDefinition,
) {
    if let Some(ref description) = field_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&field_definition.name);
    for argument in field_definition.arguments.iter() {
        visitor.visit_input_value_definition(argument);
    }
    visitor.visit_ty(&field_definition.ty);
    for directive in field_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
}

pub fn walk_fragment_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    fragment_definition: &'a FragmentDefinition,
) {
    visitor.visit_name(&fragment_definition.name);
    visitor.visit_name(&fragment_definition.type_condition);
    for directive in fragment_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
    for selection in fragment_definition.selections.iter() {
        visitor.visit_selection(selection);
    }
}

pub fn walk_fragment_spread<'a, V: Visitor<'a>>(
    visitor: &mut V,
    fragment_spread: &'a FragmentSpread,
) {
    visitor.visit_name(&fragment_spread.name);
    for directive in fragment_spread.directives.iter() {
        visitor.visit_directive(directive);
    }
}

pub fn walk_inline_fragment<'a, V: Visitor<'a>>(
    visitor: &mut V,
    inline_fragment: &'a InlineFragment,
) {
    visitor.visit_name(&inline_fragment.name);
    for directive in inline_fragment.directives.iter() {
        visitor.visit_directive(directive);
    }
    for selection in inline_fragment.selections.iter() {
        visitor.visit_selection(selection);
    }
}

pub fn walk_input_object_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    input_object_type_definition: &'a InputObjectTypeDefinition,
) {
    if let Some(ref description) = input_object_type_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&input_object_type_definition.name);
    for directive in input_object_type_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
    for argument in input_object_type_definition.fields.iter() {
        visitor.visit_input_value_definition(argument);
    }
}

pub fn walk_input_object_type_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    input_object_type_extension: &'a InputObjectTypeExtension,
) {
    visitor.visit_name(&input_object_type_extension.name);
    for directive in input_object_type_extension.directives.iter() {
        visitor.visit_directive(directive);
    }
    for argument in input_object_type_extension.values.iter() {
        visitor.visit_input_value_definition(argument);
    }
}

pub fn walk_input_value_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    input_value_definition: &'a InputValueDefinition,
) {
    if let Some(ref description) = input_value_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&input_value_definition.name);
    visitor.visit_ty(&input_value_definition.ty);
    if let Some(ref default_value) = input_value_definition.default_value {
        visitor.visit_value(default_value);
    }
    for directive in input_value_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
}

pub fn walk_interface_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    interface_type_definition: &'a InterfaceTypeDefinition,
) {
    if let Some(ref description) = interface_type_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&interface_type_definition.name);
    for interface in interface_type_definition.implement_interfaces.iter() {
        visitor.visit_name(interface);
    }
    for directive in interface_type_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
    for field in interface_type_definition.fields.iter() {
        visitor.visit_field_definition(field);
    }
}

pub fn walk_interface_type_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    interface_type_extension: &'a InterfaceTypeExtension,
) {
    visitor.visit_name(&interface_type_extension.name);
    for interface in interface_type_extension.implement_interfaces.iter() {
        visitor.visit_name(interface);
    }
    for directive in interface_type_extension.directives.iter() {
        visitor.visit_directive(directive);
    }
    for field in interface_type_extension.fields.iter() {
        visitor.visit_field_definition(field);
    }
}

pub fn walk_list_value<'a, V: Visitor<'a>>(visitor: &mut V, list_value: &'a ListValue) {
    for value in list_value.values.iter() {
        visitor.visit_value(value);
    }
}

pub fn walk_object_field<'a, V: Visitor<'a>>(visitor: &mut V, object_field: &'a ObjectField) {
    visitor.visit_name(&object_field.key);
    visitor.visit_value(&object_field.value);
}

pub fn walk_object_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    object_type_definition: &'a ObjectTypeDefinition,
) {
    visitor.visit_name(&object_type_definition.name);
    for interface in object_type_definition.implement_interfaces.iter() {
        visitor.visit_name(interface);
    }
    for directive in object_type_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
    for field in object_type_definition.fields.iter() {
        visitor.visit_field_definition(field);
    }
}

pub fn walk_object_type_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    object_type_extension: &'a ObjectTypeExtension,
) {
    visitor.visit_name(&object_type_extension.name);
    for interface in object_type_extension.implement_interfaces.iter() {
        visitor.visit_name(interface);
    }
    for directive in object_type_extension.directives.iter() {
        visitor.visit_directive(directive);
    }
    for field in object_type_extension.fields.iter() {
        visitor.visit_field_definition(field);
    }
}

pub fn walk_object_value<'a, V: Visitor<'a>>(visitor: &mut V, object_value: &'a ObjectValue) {
    for field in object_value.fields.iter() {
        visitor.visit_object_field(field);
    }
}

pub fn walk_operation_definiion<'a, V: Visitor<'a>>(
    visitor: &mut V,
    operation_definiion: &'a OperationDefinition,
) {
    visitor.visit_operation_type(&operation_definiion.operation_type);
    if let Some(ref name) = operation_definiion.name {
        visitor.visit_name(name);
    }
    for variable in operation_definiion.variables.iter() {
        visitor.visit_variable_definition(variable);
    }
    for directive in operation_definiion.directives.iter() {
        visitor.visit_directive(directive);
    }
    for selection in operation_definiion.selections.iter() {
        visitor.visit_selection(selection);
    }
}

pub fn walk_root_operation_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    root_operation_type_definition: &'a RootOperationTypeDefinition,
) {
    visitor.visit_operation_type(&root_operation_type_definition.key);
    visitor.visit_name(&root_operation_type_definition.value);
}

pub fn walk_scalar_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    scalar_type_definition: &'a ScalarTypeDefinition,
) {
    if let Some(ref description) = scalar_type_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&scalar_type_definition.name);
    for directive in scalar_type_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
}

pub fn walk_scalar_type_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    scalar_type_extension: &'a ScalarTypeExtension,
) {
    visitor.visit_name(&scalar_type_extension.name);
    for directive in scalar_type_extension.directives.iter() {
        visitor.visit_directive(directive);
    }
}

pub fn walk_schema_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    schema_definition: &'a SchemaDefinition,
) {
    if let Some(ref description) = schema_definition.description {
        visitor.visit_string_value(description);
    }
    for directive in schema_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
    for field in schema_definition.fields.iter() {
        visitor.visit_root_operation_type_definition(field);
    }
}

pub fn walk_schema_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    schema_extension: &'a SchemaExtension,
) {
    for directive in schema_extension.directives.iter() {
        visitor.visit_directive(directive);
    }
    for field in schema_extension.fields.iter() {
        visitor.visit_root_operation_type_definition(field);
    }
}

pub fn walk_selection<'a, V: Visitor<'a>>(visitor: &mut V, selection: &'a Selection) {
    match selection {
        Selection::Field(ref field) => visitor.visit_field(field),
        Selection::FragmentSpread(ref fragment_spread) => {
            visitor.visit_fragment_spread(fragment_spread);
        }
        Selection::InlineFragment(ref inline_fragment) => {
            visitor.visit_inline_fragment(inline_fragment);
        }
    }
}

pub fn walk_ty<'a, V: Visitor<'a>>(visitor: &mut V, ty: &'a Type) {
    visitor.visit_type_kind(&ty.kind);
}

pub fn walk_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    type_definition: &'a TypeDefinition,
) {
    match type_definition {
        TypeDefinition::Scalar(definition) => visitor.visit_scalar_type_definition(definition),
        TypeDefinition::Object(definition) => visitor.visit_object_type_definition(definition),
        TypeDefinition::Interface(definition) => {
            visitor.visit_interface_type_definition(definition);
        }
        TypeDefinition::Union(definition) => visitor.visit_union_type_definition(definition),
        TypeDefinition::Enum(definition) => visitor.visit_enum_type_definition(definition),
        TypeDefinition::InputObject(definition) => {
            visitor.visit_input_object_type_definition(definition);
        }
    }
}

pub fn walk_type_extension<'a, V: Visitor<'a>>(visitor: &mut V, type_extension: &'a TypeExtension) {
    match type_extension {
        TypeExtension::Scalar(extension) => visitor.visit_scalar_type_extension(extension),
        TypeExtension::Object(extension) => visitor.visit_object_type_extension(extension),
        TypeExtension::Interface(extension) => visitor.visit_interface_type_extension(extension),
        TypeExtension::Union(extension) => visitor.visit_union_type_extension(extension),
        TypeExtension::Enum(extension) => visitor.visit_enum_type_extension(extension),
        TypeExtension::InputObject(extension) => {
            visitor.visit_input_object_type_extension(extension);
        }
    }
}

pub fn walk_type_kind<'a, V: Visitor<'a>>(visitor: &mut V, type_kind: &'a TypeKind) {
    match type_kind {
        TypeKind::Named(name) => visitor.visit_name(name),
        TypeKind::List(ty) => visitor.visit_ty(ty),
        TypeKind::NonNull(ty) => visitor.visit_ty(ty),
    }
}

pub fn walk_type_system_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    type_system_definition: &'a TypeSystemDefinition,
) {
    match type_system_definition {
        TypeSystemDefinition::Schema(definition) => visitor.visit_schema_definition(definition),
        TypeSystemDefinition::Type(definition) => visitor.visit_type_definition(definition),
        TypeSystemDefinition::Directive(definition) => {
            visitor.visit_directive_definition(definition);
        }
    }
}

pub fn walk_type_system_definition_or_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    type_system_definition_or_extension: &'a TypeSystemDefinitionOrExtension,
) {
    match type_system_definition_or_extension {
        TypeSystemDefinitionOrExtension::Definition(definition) => {
            visitor.visit_type_system_definition(definition);
        }
        TypeSystemDefinitionOrExtension::Extension(definition) => {
            visitor.visit_type_system_extension(definition);
        }
    }
}

pub fn walk_type_system_directive_location<'a, V: Visitor<'a>>(
    visitor: &mut V,
    type_system_directive_location: &'a TypeSystemDirectiveLocation,
) {
    visitor.visit_type_system_directive_location_kind(&type_system_directive_location.kind);
}

pub fn walk_type_system_document<'a, V: Visitor<'a>>(
    visitor: &mut V,
    type_system_document: &'a TypeSystemDocument,
) {
    for definition in type_system_document.definitions.iter() {
        visitor.visit_type_system_definition(definition);
    }
}

pub fn walk_type_system_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    type_system_extension: &'a TypeSystemExtension,
) {
    match type_system_extension {
        TypeSystemExtension::Schema(extension) => {
            visitor.visit_schema_extension(extension);
        }
        TypeSystemExtension::Type(extension) => {
            visitor.visit_type_extension(extension);
        }
    }
}

pub fn walk_type_system_extension_document<'a, V: Visitor<'a>>(
    visitor: &mut V,
    type_system_extension_document: &'a TypeSystemExtensionDocument,
) {
    for definition in type_system_extension_document.definitions.iter() {
        visitor.visit_type_system_definition_or_extension(definition);
    }
}

pub fn walk_union_type_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    union_type_definition: &'a UnionTypeDefinition,
) {
    if let Some(ref description) = union_type_definition.description {
        visitor.visit_string_value(description);
    }
    visitor.visit_name(&union_type_definition.name);
    for directive in union_type_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
    for member in union_type_definition.members.iter() {
        visitor.visit_name(member);
    }
}

pub fn walk_union_type_extension<'a, V: Visitor<'a>>(
    visitor: &mut V,
    union_type_extension: &'a UnionTypeExtension,
) {
    visitor.visit_name(&union_type_extension.name);
    for directive in union_type_extension.directives.iter() {
        visitor.visit_directive(directive);
    }
    for member in union_type_extension.members.iter() {
        visitor.visit_name(member);
    }
}

pub fn walk_value<'a, V: Visitor<'a>>(visitor: &mut V, value: &'a Value) {
    match value {
        Value::Variable(value) => visitor.visit_variable(value),
        Value::Int(value) => visitor.visit_int_value(value),
        Value::Float(value) => visitor.visit_float_value(value),
        Value::String(value) => visitor.visit_string_value(value),
        Value::Boolean(value) => visitor.visit_boolean_value(value),
        Value::Null(value) => visitor.visit_null_value(value),
        Value::Enum(value) => visitor.visit_enum_value(value),
        Value::List(value) => visitor.visit_list_value(value),
        Value::Object(value) => visitor.visit_object_value(value),
    }
}

pub fn walk_variable<'a, V: Visitor<'a>>(visitor: &mut V, variable: &'a Variable) {
    visitor.visit_name(&variable.name);
}

pub fn walk_variable_definition<'a, V: Visitor<'a>>(
    visitor: &mut V,
    variable_definition: &'a VariableDefinition,
) {
    visitor.visit_name(&variable_definition.name);
    visitor.visit_ty(&variable_definition.ty);
    if let Some(ref default_value) = variable_definition.default_value {
        visitor.visit_value(default_value);
    }
    for directive in variable_definition.directives.iter() {
        visitor.visit_directive(directive);
    }
}
