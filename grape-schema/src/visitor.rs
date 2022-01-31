use crate::{
    GraphQLBooleanValue, GraphQLDescription, GraphQLDirective, GraphQLDirectiveLocation,
    GraphQLEnumType, GraphQLEnumValue, GraphQLField, GraphQLFloatValue, GraphQLInputField,
    GraphQLInputObjectType, GraphQLInputType, GraphQLInputValue, GraphQLIntValue,
    GraphQLInterfaceType, GraphQLListValue, GraphQLNamedInputType, GraphQLNamedOutputType,
    GraphQLNamedType, GraphQLNullValue, GraphQLObjectType, GraphQLObjectValue, GraphQLOutputType,
    GraphQLScalarType, GraphQLSchema, GraphQLStringValue, GraphQLUnionType, GraphQLValue,
};

pub trait SchemaContext {
    //
}

#[allow(unused_variables)]
pub trait SchemaVisitor<'schema, C: SchemaContext>: Sized {
    fn visit_boolean_value(&mut self, context: &C, boolean_value: &'schema GraphQLBooleanValue) {
        //
    }
    fn visit_description(&mut self, context: &C, description: &'schema GraphQLDescription) {
        //
    }
    fn visit_directive(&mut self, context: &C, directive: &'schema GraphQLDirective) {
        walk_directive(self, context, directive);
    }
    fn visit_directive_location(
        &mut self,
        context: &C,
        directive_location: &'schema GraphQLDirectiveLocation,
    ) {
        //
    }
    fn visit_enum_type(&mut self, context: &C, enum_type: &'schema GraphQLEnumType) {
        walk_enum_type(self, context, enum_type);
    }
    fn visit_enum_value(&mut self, context: &C, enum_value: &'schema GraphQLEnumValue) {
        walk_enum_value(self, context, enum_value);
    }
    fn visit_field(&mut self, context: &C, field: &'schema GraphQLField) {
        walk_field(self, context, field);
    }
    fn visit_float_value(&mut self, context: &C, float_value: &'schema GraphQLFloatValue) {
        //
    }
    fn visit_input_field(&mut self, context: &C, input_field: &'schema GraphQLInputField) {
        walk_input_field(self, context, input_field);
    }
    fn visit_input_object_type(
        &mut self,
        context: &C,
        input_object_type: &'schema GraphQLInputObjectType,
    ) {
        walk_input_object_type(self, context, input_object_type);
    }
    fn visit_input_type(&mut self, context: &C, input_type: &'schema GraphQLInputType) {
        walk_input_type(self, context, input_type);
    }
    fn visit_input_value(&mut self, context: &C, input_value: &'schema GraphQLInputValue) {
        walk_input_value(self, context, input_value);
    }
    fn visit_int_value(&mut self, context: &C, int_value: &'schema GraphQLIntValue) {
        //
    }
    fn visit_interface_type(&mut self, context: &C, interface_type: &'schema GraphQLInterfaceType) {
        walk_interface_type(self, context, interface_type);
    }
    fn visit_list_value(&mut self, context: &C, list_value: &'schema GraphQLListValue) {
        walk_list_value(self, context, list_value);
    }
    fn visit_named_input_type(
        &mut self,
        context: &C,
        named_input_type: &'schema GraphQLNamedInputType,
    ) {
        walk_named_input_type(self, context, named_input_type);
    }
    fn visit_named_output_type(
        &mut self,
        context: &C,
        named_output_type: &'schema GraphQLNamedOutputType,
    ) {
        walk_named_output_type(self, context, named_output_type);
    }
    fn visit_named_type(&mut self, context: &C, named_type: &'schema GraphQLNamedType) {
        walk_named_type(self, context, named_type);
    }
    fn visit_null_value(&mut self, context: &C, null_value: &'schema GraphQLNullValue) {
        //
    }
    fn visit_object_type(&mut self, context: &C, object_type: &'schema GraphQLObjectType) {
        walk_object_type(self, context, object_type);
    }
    fn visit_object_value(&mut self, context: &C, object_value: &'schema GraphQLObjectValue) {
        walk_object_value(self, context, object_value);
    }
    fn visit_output_type(&mut self, context: &C, output_type: &'schema GraphQLOutputType) {
        walk_output_type(self, context, output_type);
    }
    fn visit_scalar_type(&mut self, context: &C, scalar_type: &'schema GraphQLScalarType) {
        walk_scalar_type(self, context, scalar_type);
    }
    fn visit_schema(&mut self, context: &C, schema: &'schema GraphQLSchema) {
        walk_schema(self, context, schema);
    }
    fn visit_string_value(&mut self, context: &C, string_value: &'schema GraphQLStringValue) {
        //
    }
    fn visit_union_type(&mut self, context: &C, union_type: &'schema GraphQLUnionType) {
        walk_union_type(self, context, union_type);
    }
    fn visit_value(&mut self, context: &C, value: &'schema GraphQLValue) {
        walk_value(self, context, value);
    }
}

pub fn walk_directive<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    directive: &'schema GraphQLDirective,
) {
    if let Some(ref description) = directive.description {
        visitor.visit_description(context, description);
    }
    for directive_location in directive.locations.iter() {
        visitor.visit_directive_location(context, directive_location);
    }
    for value in directive.args.values() {
        visitor.visit_input_value(context, value);
    }
}

pub fn walk_enum_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    enum_type: &'schema GraphQLEnumType,
) {
    if let Some(ref description) = enum_type.description {
        visitor.visit_description(context, description);
    }
    for directive in enum_type.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for value in enum_type.values.values() {
        visitor.visit_enum_value(context, value);
    }
}

pub fn walk_enum_value<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    enum_value: &'schema GraphQLEnumValue,
) {
    if let Some(ref description) = enum_value.description {
        visitor.visit_description(context, description);
    }
    for directive in enum_value.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_field<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    field: &'schema GraphQLField,
) {
    if let Some(ref description) = field.description {
        visitor.visit_description(context, description);
    }
    for value in field.args.values() {
        visitor.visit_input_value(context, value);
    }
    visitor.visit_output_type(context, &field.ty);
    for directive in field.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_input_field<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    input_field: &'schema GraphQLInputField,
) {
    if let Some(ref description) = input_field.description {
        visitor.visit_description(context, description);
    }
    for directive in input_field.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    visitor.visit_input_type(context, &input_field.ty);
}

pub fn walk_input_object_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    input_object_type: &'schema GraphQLInputObjectType,
) {
    if let Some(ref description) = input_object_type.description {
        visitor.visit_description(context, description);
    }
    for directive in input_object_type.directives.iter() {
        visitor.visit_directive(context, directive);
    }
    for field in input_object_type.fields.values() {
        visitor.visit_input_field(context, field);
    }
}

pub fn walk_input_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    input_type: &'schema GraphQLInputType,
) {
    match input_type {
        GraphQLInputType::Named(named_input_type) => {
            visitor.visit_named_input_type(context, named_input_type)
        }
        GraphQLInputType::List(input_type) => visitor.visit_input_type(context, input_type),
        GraphQLInputType::NonNull(input_type) => visitor.visit_input_type(context, input_type),
    }
}

pub fn walk_input_value<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    input_value: &'schema GraphQLInputValue,
) {
    if let Some(ref description) = input_value.description {
        visitor.visit_description(context, description);
    }
    visitor.visit_input_type(context, &input_value.ty);
    if let Some(ref default_value) = input_value.default_value {
        visitor.visit_value(context, default_value);
    }
    for directive in input_value.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_interface_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    interface_type: &'schema GraphQLInterfaceType,
) {
    if let Some(ref description) = interface_type.description {
        visitor.visit_description(context, description);
    }
    for field in interface_type.fields.values() {
        visitor.visit_field(context, field);
    }
    for directive in interface_type.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_list_value<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    list_value: &'schema GraphQLListValue,
) {
    for value in list_value.values.iter() {
        visitor.visit_value(context, value);
    }
}

pub fn walk_named_input_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    named_input_type: &'schema GraphQLNamedInputType,
) {
    match named_input_type {
        GraphQLNamedInputType::Scalar(ty) => visitor.visit_scalar_type(context, ty),
        GraphQLNamedInputType::Enum(ty) => visitor.visit_enum_type(context, ty),
        GraphQLNamedInputType::InputObject(ty) => visitor.visit_input_object_type(context, ty),
    }
}

pub fn walk_named_output_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    named_output_type: &'schema GraphQLNamedOutputType,
) {
    match named_output_type {
        GraphQLNamedOutputType::Scalar(ty) => visitor.visit_scalar_type(context, ty),
        GraphQLNamedOutputType::Object(ty) => visitor.visit_object_type(context, ty),
        GraphQLNamedOutputType::Interface(ty) => visitor.visit_interface_type(context, ty),
        GraphQLNamedOutputType::Union(ty) => visitor.visit_union_type(context, ty),
        GraphQLNamedOutputType::Enum(ty) => visitor.visit_enum_type(context, ty),
    }
}

pub fn walk_named_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    named_type: &'schema GraphQLNamedType,
) {
    match named_type {
        GraphQLNamedType::Scalar(ty) => visitor.visit_scalar_type(context, ty),
        GraphQLNamedType::Object(ty) => visitor.visit_object_type(context, ty),
        GraphQLNamedType::Interface(ty) => visitor.visit_interface_type(context, ty),
        GraphQLNamedType::Union(ty) => visitor.visit_union_type(context, ty),
        GraphQLNamedType::Enum(ty) => visitor.visit_enum_type(context, ty),
        GraphQLNamedType::InputObject(ty) => visitor.visit_input_object_type(context, ty),
    }
}

pub fn walk_object_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    object_type: &'schema GraphQLObjectType,
) {
    if let Some(ref description) = object_type.description {
        visitor.visit_description(context, description);
    }
    for interface in object_type.interfaces.iter() {
        visitor.visit_interface_type(context, interface);
    }
    for field in object_type.fields.values() {
        visitor.visit_field(context, field);
    }
    for directive in object_type.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_object_value<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    object_value: &'schema GraphQLObjectValue,
) {
    for value in object_value.fields.values() {
        visitor.visit_value(context, value);
    }
}

pub fn walk_output_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    output_type: &'schema GraphQLOutputType,
) {
    match output_type {
        GraphQLOutputType::Named(ty) => visitor.visit_named_output_type(context, ty),
        GraphQLOutputType::List(ty) => visitor.visit_output_type(context, ty),
        GraphQLOutputType::NonNull(ty) => visitor.visit_output_type(context, ty),
    }
}

pub fn walk_scalar_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    scalar_type: &'schema GraphQLScalarType,
) {
    if let Some(ref description) = scalar_type.description {
        visitor.visit_description(context, description);
    }
    for directive in scalar_type.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_schema<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    schema: &'schema GraphQLSchema,
) {
    visitor.visit_object_type(context, &schema.query);
    if let Some(ref mutation) = schema.mutation {
        visitor.visit_object_type(context, mutation);
    }
    if let Some(ref subscription) = schema.subscription {
        visitor.visit_object_type(context, subscription);
    }
    for directive in schema.directives.values() {
        visitor.visit_directive(context, directive);
    }
    for ty in schema.types.values() {
        visitor.visit_named_type(context, ty);
    }
}

pub fn walk_union_type<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    union_type: &'schema GraphQLUnionType,
) {
    if let Some(ref description) = union_type.description {
        visitor.visit_description(context, description);
    }
    for directive in union_type.directives.iter() {
        visitor.visit_directive(context, directive);
    }
}

pub fn walk_value<'schema, C: SchemaContext, V: SchemaVisitor<'schema, C>>(
    visitor: &mut V,
    context: &C,
    value: &'schema GraphQLValue,
) {
    match value {
        GraphQLValue::Null(value) => visitor.visit_null_value(context, value),
        GraphQLValue::Int(value) => visitor.visit_int_value(context, value),
        GraphQLValue::Float(value) => visitor.visit_float_value(context, value),
        GraphQLValue::String(value) => visitor.visit_string_value(context, value),
        GraphQLValue::Boolean(value) => visitor.visit_boolean_value(context, value),
        GraphQLValue::Enum(value) => visitor.visit_enum_value(context, value),
        GraphQLValue::List(value) => visitor.visit_list_value(context, value),
        GraphQLValue::Object(value) => visitor.visit_object_value(context, value),
    }
}
