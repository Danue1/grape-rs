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
use grape_diagnostics::Message;
use grape_span::Span;
use grape_symbol::Symbol;

pub trait DocumentContext {
    fn span(&self, span: Span) -> &str;
    fn report(&self, message: Message);
    fn fragment(&self, symbol: Symbol) -> Option<&FragmentDefinition>;
    fn variable(&self, symbol: Symbol) -> Option<&Value>;
}

#[allow(unused_variables)]
pub trait DocumentVisitor<'document, C: DocumentContext>: Sized {
    fn visit_argument(&mut self, context: &C, argument: &'document Argument) {
        walk_argument(self, context, argument);
    }
    fn visit_boolean_value(&mut self, context: &C, boolean_value: &'document BooleanValue) {
        //
    }
    fn visit_definition(&mut self, context: &C, definition: &'document Definition) {
        walk_definition(self, context, definition);
    }
    fn visit_directive(&mut self, context: &C, directive: &'document Directive) {
        walk_directive(self, context, directive);
    }
    fn visit_directive_definition(
        &mut self,
        context: &C,
        directive_definition: &'document DirectiveDefinition,
    ) {
        walk_directive_definition(self, context, directive_definition);
    }
    fn visit_directive_location(
        &mut self,
        context: &C,
        directive_location: &'document DirectiveLocation,
    ) {
        walk_directive_location(self, context, directive_location);
    }
    fn visit_document(&mut self, context: &C, document: &'document Document) {
        walk_document(self, context, document);
    }
    fn visit_enum_type_definition(
        &mut self,
        context: &C,
        enum_type_definition: &'document EnumTypeDefinition,
    ) {
        walk_enum_type_definition(self, context, enum_type_definition);
    }
    fn visit_enum_type_extension(
        &mut self,
        context: &C,
        enum_type_extension: &'document EnumTypeExtension,
    ) {
        walk_enum_type_extension(self, context, enum_type_extension);
    }
    fn visit_enum_value(&mut self, context: &C, enum_value: &'document EnumValue) {
        walk_enum_value(self, context, enum_value);
    }
    fn visit_enum_value_definition(
        &mut self,
        context: &C,
        enum_value_definition: &'document EnumValueDefinition,
    ) {
        walk_enum_value_definition(self, context, enum_value_definition);
    }
    fn visit_executable_definition(
        &mut self,
        context: &C,
        executable_definition: &'document ExecutableDefinition,
    ) {
        walk_executable_definition(self, context, executable_definition);
    }
    fn visit_executable_directive_location(
        &mut self,
        context: &C,
        executable_directive_location: &'document ExecutableDirectiveLocation,
    ) {
        walk_executable_directive_location(self, context, executable_directive_location);
    }
    fn visit_executable_directive_location_kind(
        &mut self,
        context: &C,
        executable_directive_location_kind: &'document ExecutableDirectiveLocationKind,
    ) {
        //
    }
    fn visit_executable_document(
        &mut self,
        context: &C,
        executable_document: &'document ExecutableDocument,
    ) {
        walk_executable_document(self, context, executable_document);
    }
    fn visit_field(&mut self, context: &C, field: &'document Field) {
        walk_field(self, context, field);
    }
    fn visit_field_definition(
        &mut self,
        context: &C,
        field_definition: &'document FieldDefinition,
    ) {
        walk_field_definition(self, context, field_definition);
    }
    fn visit_float_value(&mut self, context: &C, float_value: &'document FloatValue) {
        //
    }
    fn visit_fragment_definition(
        &mut self,
        context: &C,
        fragment_definition: &'document FragmentDefinition,
    ) {
        walk_fragment_definition(self, context, fragment_definition);
    }
    fn visit_fragment_spread(&mut self, context: &C, fragment_spread: &'document FragmentSpread) {
        walk_fragment_spread(self, context, fragment_spread);
    }
    fn visit_inline_fragment(&mut self, context: &C, inline_fragment: &'document InlineFragment) {
        walk_inline_fragment(self, context, inline_fragment);
    }
    fn visit_input_object_type_definition(
        &mut self,
        context: &C,
        input_object_type_definition: &'document InputObjectTypeDefinition,
    ) {
        walk_input_object_type_definition(self, context, input_object_type_definition);
    }
    fn visit_input_object_type_extension(
        &mut self,
        context: &C,
        input_object_type_extension: &'document InputObjectTypeExtension,
    ) {
        walk_input_object_type_extension(self, context, input_object_type_extension);
    }
    fn visit_input_value_definition(
        &mut self,
        context: &C,
        input_value_definition: &'document InputValueDefinition,
    ) {
        walk_input_value_definition(self, context, input_value_definition);
    }
    fn visit_int_value(&mut self, context: &C, int_value: &'document IntValue) {
        //
    }
    fn visit_interface_type_definition(
        &mut self,
        context: &C,
        interface_type_definition: &'document InterfaceTypeDefinition,
    ) {
        walk_interface_type_definition(self, context, interface_type_definition);
    }
    fn visit_interface_type_extension(
        &mut self,
        context: &C,
        interface_type_extension: &'document InterfaceTypeExtension,
    ) {
        walk_interface_type_extension(self, context, interface_type_extension);
    }
    fn visit_list_value(&mut self, context: &C, list_value: &'document ListValue) {
        walk_list_value(self, context, list_value);
    }
    fn visit_name(&mut self, context: &C, name: &'document Name) {
        //
    }
    fn visit_null_value(&mut self, context: &C, null_value: &'document NullValue) {
        //
    }
    fn visit_object_field(&mut self, context: &C, object_field: &'document ObjectField) {
        walk_object_field(self, context, object_field);
    }
    fn visit_object_type_definition(
        &mut self,
        context: &C,
        object_type_definition: &'document ObjectTypeDefinition,
    ) {
        walk_object_type_definition(self, context, object_type_definition);
    }
    fn visit_object_type_extension(
        &mut self,
        context: &C,
        object_type_extension: &'document ObjectTypeExtension,
    ) {
        walk_object_type_extension(self, context, object_type_extension);
    }
    fn visit_object_value(&mut self, context: &C, object_value: &'document ObjectValue) {
        walk_object_value(self, context, object_value);
    }
    fn visit_operation_definition(
        &mut self,
        context: &C,
        operation_definition: &'document OperationDefinition,
    ) {
        walk_operation_definition(self, context, operation_definition);
    }
    fn visit_operation_type(&mut self, context: &C, operation_type: &'document OperationType) {
        //
    }
    fn visit_root_operation_type_definition(
        &mut self,
        context: &C,
        root_operation_type_definition: &'document RootOperationTypeDefinition,
    ) {
        walk_root_operation_type_definition(self, context, root_operation_type_definition);
    }
    fn visit_scalar_type_definition(
        &mut self,
        context: &C,
        scalar_type_definition: &'document ScalarTypeDefinition,
    ) {
        walk_scalar_type_definition(self, context, scalar_type_definition);
    }
    fn visit_scalar_type_extension(
        &mut self,
        context: &C,
        scalar_type_extension: &'document ScalarTypeExtension,
    ) {
        walk_scalar_type_extension(self, context, scalar_type_extension);
    }
    fn visit_schema_definition(
        &mut self,
        context: &C,
        schema_definition: &'document SchemaDefinition,
    ) {
        walk_schema_definition(self, context, schema_definition);
    }
    fn visit_schema_extension(
        &mut self,
        context: &C,
        schema_extension: &'document SchemaExtension,
    ) {
        walk_schema_extension(self, context, schema_extension);
    }
    fn visit_selection(&mut self, context: &C, selection: &'document Selection) {
        walk_selection(self, context, selection);
    }
    fn visit_string_value(&mut self, context: &C, string_value: &'document StringValue) {
        //
    }
    fn visit_ty(&mut self, context: &C, ty: &'document Type) {
        walk_ty(self, context, ty);
    }
    fn visit_type_definition(&mut self, context: &C, type_definition: &'document TypeDefinition) {
        walk_type_definition(self, context, type_definition);
    }
    fn visit_type_extension(&mut self, context: &C, type_extension: &'document TypeExtension) {
        walk_type_extension(self, context, type_extension);
    }
    fn visit_type_kind(&mut self, context: &C, type_kind: &'document TypeKind) {
        walk_type_kind(self, context, type_kind);
    }
    fn visit_type_system_definition(
        &mut self,
        context: &C,
        type_system_definition: &'document TypeSystemDefinition,
    ) {
        walk_type_system_definition(self, context, type_system_definition);
    }
    fn visit_type_system_definition_or_extension(
        &mut self,
        context: &C,
        type_system_definition_or_extension: &'document TypeSystemDefinitionOrExtension,
    ) {
        walk_type_system_definition_or_extension(
            self,
            context,
            type_system_definition_or_extension,
        );
    }
    fn visit_type_system_directive_location(
        &mut self,
        context: &C,
        type_system_directive_location: &'document TypeSystemDirectiveLocation,
    ) {
        walk_type_system_directive_location(self, context, type_system_directive_location);
    }
    fn visit_type_system_directive_location_kind(
        &mut self,
        context: &C,
        type_system_directive_location_kind: &'document TypeSystemDirectiveLocationKind,
    ) {
        //
    }
    fn visit_type_system_document(
        &mut self,
        context: &C,
        type_system_document: &'document TypeSystemDocument,
    ) {
        walk_type_system_document(self, context, type_system_document);
    }
    fn visit_type_system_extension(
        &mut self,
        context: &C,
        type_system_extension: &'document TypeSystemExtension,
    ) {
        walk_type_system_extension(self, context, type_system_extension);
    }
    fn visit_type_system_extension_document(
        &mut self,
        context: &C,
        type_system_extension_document: &'document TypeSystemExtensionDocument,
    ) {
        walk_type_system_extension_document(self, context, type_system_extension_document);
    }
    fn visit_union_type_definition(
        &mut self,
        context: &C,
        union_type_definition: &'document UnionTypeDefinition,
    ) {
        walk_union_type_definition(self, context, union_type_definition);
    }
    fn visit_union_type_extension(
        &mut self,
        context: &C,
        union_type_extension: &'document UnionTypeExtension,
    ) {
        walk_union_type_extension(self, context, union_type_extension);
    }
    fn visit_value(&mut self, context: &C, value: &'document Value) {
        walk_value(self, context, value);
    }
    fn visit_variable(&mut self, context: &C, variable: &'document Variable) {
        walk_variable(self, context, variable);
    }
    fn visit_variable_definition(
        &mut self,
        context: &C,
        variable_definition: &'document VariableDefinition,
    ) {
        walk_variable_definition(self, context, variable_definition);
    }
}

pub fn walk_argument<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    argument: &'document Argument,
) {
    visitor.visit_name(context, &argument.key);
    visitor.visit_value(context, &argument.value);
}

pub fn walk_definition<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    definition: &'document Definition,
) {
    match definition {
        Definition::Operation(ref definition) => {
            visitor.visit_operation_definition(context, definition)
        }
        Definition::Fragment(ref definition) => {
            visitor.visit_fragment_definition(context, definition)
        }
        Definition::TypeSystem(ref definition) => {
            visitor.visit_type_system_definition(context, definition)
        }
        Definition::Extension(ref definition) => {
            visitor.visit_type_system_extension(context, definition)
        }
    }
}

pub fn walk_directive<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    directive: &'document Directive,
) {
    visitor.visit_name(context, &directive.name);
    for argument in directive.arguments.iter() {
        visitor.visit_argument(context, argument);
    }
}

pub fn walk_directive_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    directive_definition: &'document DirectiveDefinition,
) {
    if let Some(ref description) = directive_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &directive_definition.name);
    for argument in directive_definition.arguments.iter() {
        visitor.visit_input_value_definition(context, argument);
    }
    for location in directive_definition.locations.iter() {
        visitor.visit_directive_location(context, location);
    }
}

pub fn walk_directive_location<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    directive_location: &'document DirectiveLocation,
) {
    match directive_location {
        DirectiveLocation::Executable(location) => {
            visitor.visit_executable_directive_location(context, location);
        }
        DirectiveLocation::TypeSystem(location) => {
            visitor.visit_type_system_directive_location(context, location);
        }
    }
}

pub fn walk_document<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    document: &'document Document,
) {
    for definition in document.definitions.iter() {
        visitor.visit_definition(context, definition);
    }
}

pub fn walk_enum_type_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    enum_type_definition: &'document EnumTypeDefinition,
) {
    if let Some(ref description) = enum_type_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &enum_type_definition.name);
    for directive in enum_type_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for value in enum_type_definition.values.iter() {
        visitor.visit_enum_value_definition(context, value);
    }
}

pub fn walk_enum_type_extension<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    enum_type_extension: &'document EnumTypeExtension,
) {
    visitor.visit_name(context, &enum_type_extension.name);
    for directive in enum_type_extension.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for value in enum_type_extension.values.iter() {
        visitor.visit_enum_value_definition(context, value);
    }
}

pub fn walk_enum_value<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    enum_value: &'document EnumValue,
) {
    visitor.visit_name(context, &enum_value.name);
}

pub fn walk_enum_value_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    enum_value_definition: &'document EnumValueDefinition,
) {
    if let Some(ref description) = enum_value_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &enum_value_definition.name);
    for directive in enum_value_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_executable_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    executable_definition: &'document ExecutableDefinition,
) {
    match executable_definition {
        ExecutableDefinition::Operation(definition) => {
            visitor.visit_operation_definition(context, definition);
        }
        ExecutableDefinition::Fragment(definition) => {
            visitor.visit_fragment_definition(context, definition)
        }
    }
}

pub fn walk_executable_directive_location<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    executable_directive_location: &'document ExecutableDirectiveLocation,
) {
    visitor.visit_executable_directive_location_kind(context, &executable_directive_location.kind);
}

pub fn walk_executable_document<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    executable_document: &'document ExecutableDocument,
) {
    for definition in executable_document.definitions.iter() {
        visitor.visit_executable_definition(context, definition);
    }
}

pub fn walk_field<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    field: &'document Field,
) {
    if let Some(ref alias) = field.alias {
        visitor.visit_name(context, alias);
    }
    visitor.visit_name(context, &field.name);
    for argument in field.arguments.iter() {
        visitor.visit_argument(context, argument);
    }
    for directive in field.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for selection in field.selections.iter() {
        visitor.visit_selection(context, selection);
    }
}

pub fn walk_field_definition<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    field_definition: &'document FieldDefinition,
) {
    if let Some(ref description) = field_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &field_definition.name);
    for argument in field_definition.arguments.iter() {
        visitor.visit_input_value_definition(context, argument);
    }
    visitor.visit_ty(context, &field_definition.ty);
    for directive in field_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_fragment_definition<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    fragment_definition: &'document FragmentDefinition,
) {
    visitor.visit_name(context, &fragment_definition.name);
    visitor.visit_name(context, &fragment_definition.type_condition);
    for directive in fragment_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for selection in fragment_definition.selections.iter() {
        visitor.visit_selection(context, selection);
    }
}

pub fn walk_fragment_spread<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    fragment_spread: &'document FragmentSpread,
) {
    visitor.visit_name(context, &fragment_spread.name);
    for directive in fragment_spread.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_inline_fragment<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    inline_fragment: &'document InlineFragment,
) {
    visitor.visit_name(context, &inline_fragment.name);
    for directive in inline_fragment.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for selection in inline_fragment.selections.iter() {
        visitor.visit_selection(context, selection);
    }
}

pub fn walk_input_object_type_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    input_object_type_definition: &'document InputObjectTypeDefinition,
) {
    if let Some(ref description) = input_object_type_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &input_object_type_definition.name);
    for directive in input_object_type_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for argument in input_object_type_definition.fields.iter() {
        visitor.visit_input_value_definition(context, argument);
    }
}

pub fn walk_input_object_type_extension<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    input_object_type_extension: &'document InputObjectTypeExtension,
) {
    visitor.visit_name(context, &input_object_type_extension.name);
    for directive in input_object_type_extension.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for argument in input_object_type_extension.values.iter() {
        visitor.visit_input_value_definition(context, argument);
    }
}

pub fn walk_input_value_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    input_value_definition: &'document InputValueDefinition,
) {
    if let Some(ref description) = input_value_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &input_value_definition.name);
    visitor.visit_ty(context, &input_value_definition.ty);
    if let Some(ref default_value) = input_value_definition.default_value {
        visitor.visit_value(context, default_value);
    }
    for directive in input_value_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_interface_type_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    interface_type_definition: &'document InterfaceTypeDefinition,
) {
    if let Some(ref description) = interface_type_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &interface_type_definition.name);
    for interface in interface_type_definition.implement_interfaces.iter() {
        visitor.visit_name(context, interface);
    }
    for directive in interface_type_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for field in interface_type_definition.fields.iter() {
        visitor.visit_field_definition(context, field);
    }
}

pub fn walk_interface_type_extension<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    interface_type_extension: &'document InterfaceTypeExtension,
) {
    visitor.visit_name(context, &interface_type_extension.name);
    for interface in interface_type_extension.implement_interfaces.iter() {
        visitor.visit_name(context, interface);
    }
    for directive in interface_type_extension.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for field in interface_type_extension.fields.iter() {
        visitor.visit_field_definition(context, field);
    }
}

pub fn walk_list_value<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    list_value: &'document ListValue,
) {
    for value in list_value.values.iter() {
        visitor.visit_value(context, value);
    }
}

pub fn walk_object_field<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    object_field: &'document ObjectField,
) {
    visitor.visit_name(context, &object_field.key);
    visitor.visit_value(context, &object_field.value);
}

pub fn walk_object_type_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    object_type_definition: &'document ObjectTypeDefinition,
) {
    visitor.visit_name(context, &object_type_definition.name);
    for interface in object_type_definition.implement_interfaces.iter() {
        visitor.visit_name(context, interface);
    }
    for directive in object_type_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for field in object_type_definition.fields.iter() {
        visitor.visit_field_definition(context, field);
    }
}

pub fn walk_object_type_extension<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    object_type_extension: &'document ObjectTypeExtension,
) {
    visitor.visit_name(context, &object_type_extension.name);
    for interface in object_type_extension.implement_interfaces.iter() {
        visitor.visit_name(context, interface);
    }
    for directive in object_type_extension.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for field in object_type_extension.fields.iter() {
        visitor.visit_field_definition(context, field);
    }
}

pub fn walk_object_value<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    object_value: &'document ObjectValue,
) {
    for field in object_value.fields.iter() {
        visitor.visit_object_field(context, field);
    }
}

pub fn walk_operation_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    operation_definition: &'document OperationDefinition,
) {
    visitor.visit_operation_type(context, &operation_definition.operation);
    if let Some(ref name) = operation_definition.name {
        visitor.visit_name(context, name);
    }
    for variable in operation_definition.variables.iter() {
        visitor.visit_variable_definition(context, variable);
    }
    for directive in operation_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for selection in operation_definition.selections.iter() {
        visitor.visit_selection(context, selection);
    }
}

pub fn walk_root_operation_type_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    root_operation_type_definition: &'document RootOperationTypeDefinition,
) {
    visitor.visit_operation_type(context, &root_operation_type_definition.key);
    visitor.visit_name(context, &root_operation_type_definition.value);
}

pub fn walk_scalar_type_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    scalar_type_definition: &'document ScalarTypeDefinition,
) {
    if let Some(ref description) = scalar_type_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &scalar_type_definition.name);
    for directive in scalar_type_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_scalar_type_extension<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    scalar_type_extension: &'document ScalarTypeExtension,
) {
    visitor.visit_name(context, &scalar_type_extension.name);
    for directive in scalar_type_extension.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_schema_definition<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    schema_definition: &'document SchemaDefinition,
) {
    if let Some(ref description) = schema_definition.description {
        visitor.visit_string_value(context, description);
    }
    for directive in schema_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for field in schema_definition.fields.iter() {
        visitor.visit_root_operation_type_definition(context, field);
    }
}

pub fn walk_schema_extension<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    schema_extension: &'document SchemaExtension,
) {
    for directive in schema_extension.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for field in schema_extension.fields.iter() {
        visitor.visit_root_operation_type_definition(context, field);
    }
}

pub fn walk_selection<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    selection: &'document Selection,
) {
    match selection {
        Selection::Field(ref field) => visitor.visit_field(context, field),
        Selection::FragmentSpread(ref fragment_spread) => {
            visitor.visit_fragment_spread(context, fragment_spread);
        }
        Selection::InlineFragment(ref inline_fragment) => {
            visitor.visit_inline_fragment(context, inline_fragment);
        }
    }
}

pub fn walk_ty<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    ty: &'document Type,
) {
    visitor.visit_type_kind(context, &ty.kind);
}

pub fn walk_type_definition<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    type_definition: &'document TypeDefinition,
) {
    match type_definition {
        TypeDefinition::Scalar(definition) => {
            visitor.visit_scalar_type_definition(context, definition)
        }
        TypeDefinition::Object(definition) => {
            visitor.visit_object_type_definition(context, definition)
        }
        TypeDefinition::Interface(definition) => {
            visitor.visit_interface_type_definition(context, definition);
        }
        TypeDefinition::Union(definition) => {
            visitor.visit_union_type_definition(context, definition)
        }
        TypeDefinition::Enum(definition) => visitor.visit_enum_type_definition(context, definition),
        TypeDefinition::InputObject(definition) => {
            visitor.visit_input_object_type_definition(context, definition);
        }
    }
}

pub fn walk_type_extension<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    type_extension: &'document TypeExtension,
) {
    match type_extension {
        TypeExtension::Scalar(extension) => visitor.visit_scalar_type_extension(context, extension),
        TypeExtension::Object(extension) => visitor.visit_object_type_extension(context, extension),
        TypeExtension::Interface(extension) => {
            visitor.visit_interface_type_extension(context, extension)
        }
        TypeExtension::Union(extension) => visitor.visit_union_type_extension(context, extension),
        TypeExtension::Enum(extension) => visitor.visit_enum_type_extension(context, extension),
        TypeExtension::InputObject(extension) => {
            visitor.visit_input_object_type_extension(context, extension);
        }
    }
}

pub fn walk_type_kind<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    type_kind: &'document TypeKind,
) {
    match type_kind {
        TypeKind::Named(name) => visitor.visit_name(context, name),
        TypeKind::List(ty) => visitor.visit_ty(context, ty),
        TypeKind::NonNull(ty) => visitor.visit_ty(context, ty),
    }
}

pub fn walk_type_system_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    type_system_definition: &'document TypeSystemDefinition,
) {
    match type_system_definition {
        TypeSystemDefinition::Schema(definition) => {
            visitor.visit_schema_definition(context, definition)
        }
        TypeSystemDefinition::Type(definition) => {
            visitor.visit_type_definition(context, definition)
        }
        TypeSystemDefinition::Directive(definition) => {
            visitor.visit_directive_definition(context, definition);
        }
    }
}

pub fn walk_type_system_definition_or_extension<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    type_system_definition_or_extension: &'document TypeSystemDefinitionOrExtension,
) {
    match type_system_definition_or_extension {
        TypeSystemDefinitionOrExtension::Definition(definition) => {
            visitor.visit_type_system_definition(context, definition);
        }
        TypeSystemDefinitionOrExtension::Extension(definition) => {
            visitor.visit_type_system_extension(context, definition);
        }
    }
}

pub fn walk_type_system_directive_location<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    type_system_directive_location: &'document TypeSystemDirectiveLocation,
) {
    visitor
        .visit_type_system_directive_location_kind(context, &type_system_directive_location.kind);
}

pub fn walk_type_system_document<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    type_system_document: &'document TypeSystemDocument,
) {
    for definition in type_system_document.definitions.iter() {
        visitor.visit_type_system_definition(context, definition);
    }
}

pub fn walk_type_system_extension<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    type_system_extension: &'document TypeSystemExtension,
) {
    match type_system_extension {
        TypeSystemExtension::Schema(extension) => {
            visitor.visit_schema_extension(context, extension);
        }
        TypeSystemExtension::Type(extension) => {
            visitor.visit_type_extension(context, extension);
        }
    }
}

pub fn walk_type_system_extension_document<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    type_system_extension_document: &'document TypeSystemExtensionDocument,
) {
    for definition in type_system_extension_document.definitions.iter() {
        visitor.visit_type_system_definition_or_extension(context, definition);
    }
}

pub fn walk_union_type_definition<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    union_type_definition: &'document UnionTypeDefinition,
) {
    if let Some(ref description) = union_type_definition.description {
        visitor.visit_string_value(context, description);
    }
    visitor.visit_name(context, &union_type_definition.name);
    for directive in union_type_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for member in union_type_definition.members.iter() {
        visitor.visit_name(context, member);
    }
}

pub fn walk_union_type_extension<
    'document,
    C: DocumentContext,
    V: DocumentVisitor<'document, C>,
>(
    visitor: &mut V,
    context: &C,
    union_type_extension: &'document UnionTypeExtension,
) {
    visitor.visit_name(context, &union_type_extension.name);
    for directive in union_type_extension.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for member in union_type_extension.members.iter() {
        visitor.visit_name(context, member);
    }
}

pub fn walk_value<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    value: &'document Value,
) {
    match value {
        Value::Variable(value) => visitor.visit_variable(context, value),
        Value::Int(value) => visitor.visit_int_value(context, value),
        Value::Float(value) => visitor.visit_float_value(context, value),
        Value::String(value) => visitor.visit_string_value(context, value),
        Value::Boolean(value) => visitor.visit_boolean_value(context, value),
        Value::Null(value) => visitor.visit_null_value(context, value),
        Value::Enum(value) => visitor.visit_enum_value(context, value),
        Value::List(value) => visitor.visit_list_value(context, value),
        Value::Object(value) => visitor.visit_object_value(context, value),
    }
}

pub fn walk_variable<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    variable: &'document Variable,
) {
    visitor.visit_name(context, &variable.name);
}

pub fn walk_variable_definition<'document, C: DocumentContext, V: DocumentVisitor<'document, C>>(
    visitor: &mut V,
    context: &C,
    variable_definition: &'document VariableDefinition,
) {
    visitor.visit_name(context, &variable_definition.name);
    visitor.visit_ty(context, &variable_definition.ty);
    if let Some(ref default_value) = variable_definition.default_value {
        visitor.visit_value(context, default_value);
    }
    for directive in variable_definition.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}
