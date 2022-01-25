use crate::{Directive, Name, OperationType, StringValue, Type, Value};
use grape_span::Span;

/// [Spec](https://spec.graphql.org/October2021/#TypeSystemDefinition)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeSystemDefinition {
    Schema(SchemaDefinition),
    Type(TypeDefinition),
    Directive(DirectiveDefinition),
}

impl TypeSystemDefinition {
    pub const fn span(&self) -> Span {
        match self {
            TypeSystemDefinition::Schema(definition) => definition.span,
            TypeSystemDefinition::Type(definition) => definition.span(),
            TypeSystemDefinition::Directive(definition) => definition.span,
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#SchemaDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct SchemaDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub directives: Vec<Directive>,
    pub fields: Vec<RootOperationTypeDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#RootOperationTypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct RootOperationTypeDefinition {
    pub span: Span,
    pub key: OperationType,
    pub value: Name,
}

/// [Spec](https://spec.graphql.org/October2021/#TypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeDefinition {
    Scalar(ScalarTypeDefinition),
    Object(ObjectTypeDefinition),
    Interface(InterfaceTypeDefinition),
    Union(UnionTypeDefinition),
    Enum(EnumTypeDefinition),
    InputObject(InputObjectTypeDefinition),
}

impl TypeDefinition {
    pub const fn span(&self) -> Span {
        match self {
            TypeDefinition::Scalar(definition) => definition.span,
            TypeDefinition::Object(definition) => definition.span,
            TypeDefinition::Interface(definition) => definition.span,
            TypeDefinition::Union(definition) => definition.span,
            TypeDefinition::Enum(definition) => definition.span,
            TypeDefinition::InputObject(definition) => definition.span,
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#ScalarTypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct ScalarTypeDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub directives: Vec<Directive>,
}

/// [Spec](https://spec.graphql.org/October2021/#ObjectTypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectTypeDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub implement_interfaces: Vec<Name>,
    pub directives: Vec<Directive>,
    pub fields: Vec<FieldDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#FieldDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct FieldDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub arguments: Vec<InputValueDefinition>,
    pub ty: Type,
    pub directives: Vec<Directive>,
}

/// [Spec](https://spec.graphql.org/October2021/#InterfaceTypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub implement_interfaces: Vec<Name>,
    pub directives: Vec<Directive>,
    pub fields: Vec<FieldDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#UnionTypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct UnionTypeDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub members: Vec<Name>,
}

/// [Spec](https://spec.graphql.org/October2021/#EnumTypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct EnumTypeDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub values: Vec<EnumValueDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#EnumValueDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct EnumValueDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub directives: Vec<Directive>,
}

/// [Spec](https://spec.graphql.org/October2021/#InputObjectTypeDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct InputObjectTypeDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub fields: Vec<InputValueDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#InputValueDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct InputValueDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub ty: Type,
    pub default_value: Option<Value>,
    pub directives: Vec<Directive>,
}

/// [Spec](https://spec.graphql.org/October2021/#DirectiveDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct DirectiveDefinition {
    pub span: Span,
    pub description: Option<StringValue>,
    pub name: Name,
    pub arguments: Vec<InputValueDefinition>,
    pub is_repeatable: bool,
    pub locations: Vec<DirectiveLocation>,
}

/// [Spec](https://spec.graphql.org/October2021/#DirectiveLocation)
#[derive(Debug, PartialEq, Clone)]
pub enum DirectiveLocation {
    Executable(ExecutableDirectiveLocation),
    TypeSystem(TypeSystemDirectiveLocation),
}

impl DirectiveLocation {
    pub const fn span(&self) -> Span {
        match self {
            DirectiveLocation::Executable(location) => location.span,
            DirectiveLocation::TypeSystem(location) => location.span,
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#ExecutableDirectiveLocation)
#[derive(Debug, PartialEq, Clone)]
pub struct ExecutableDirectiveLocation {
    pub span: Span,
    pub kind: ExecutableDirectiveLocationKind,
}

/// [Spec](https://spec.graphql.org/October2021/#ExecutableDirectiveLocation)
#[derive(Debug, PartialEq, Clone)]
pub enum ExecutableDirectiveLocationKind {
    /// QUERY
    Query,

    /// MUTATION
    Mutation,

    /// SUBSCRIPTION
    Subscription,

    /// FIELD
    Field,

    /// FRAGMENT_DEFINITION
    FragmentDefinition,

    /// FRAGMENT_SPREAD
    FragmentSpread,

    /// INLINE_FRAGMENT
    InlineFragment,

    /// VARIABLE_DEFINITION
    VariableDefinition,
}

/// [Spec](https://spec.graphql.org/October2021/#TypeSystemDirectiveLocation)
#[derive(Debug, PartialEq, Clone)]
pub struct TypeSystemDirectiveLocation {
    pub span: Span,
    pub kind: TypeSystemDirectiveLocationKind,
}

/// [Spec](https://spec.graphql.org/October2021/#TypeSystemDirectiveLocation)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeSystemDirectiveLocationKind {
    /// SCHEMA
    Schema,

    /// SCALAR
    Scalar,

    /// OBJECT
    Object,

    /// FIELD_DEFINITION
    FieldDefinition,

    /// ARGUMENT_DEFINITION
    ArgumentDefinition,

    /// INTERFACE
    Interface,

    /// UNION
    Union,

    /// ENUM
    Enum,

    /// ENUM_VALUE
    EnumValue,

    /// INPUT_OBJECT
    InputObject,

    /// INPUT_FIELD_DEFINITION
    InputFieldDefinition,
}

/// [Spec](https://spec.graphql.org/October2021/#TypeSystemExtension)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeSystemExtension {
    Schema(SchemaExtension),
    Type(TypeExtension),
}

impl TypeSystemExtension {
    pub const fn span(&self) -> Span {
        match self {
            TypeSystemExtension::Schema(extension) => extension.span,
            TypeSystemExtension::Type(extension) => extension.span(),
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#SchemaExtension)
#[derive(Debug, PartialEq, Clone)]
pub struct SchemaExtension {
    pub span: Span,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub definitions: Vec<(OperationType, Name)>,
}

/// [Spec](https://spec.graphql.org/October2021/#TypeExtension)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeExtension {
    Scalar(ScalarTypeExtension),
    Object(ObjectTypeExtension),
    Interface(InterfaceTypeExtension),
    Union(UnionTypeExtension),
    Enum(EnumTypeExtension),
    InputObject(InputObjectTypeExtension),
}

impl TypeExtension {
    pub const fn span(&self) -> Span {
        match self {
            TypeExtension::Scalar(extension) => extension.span,
            TypeExtension::Object(extension) => extension.span,
            TypeExtension::Interface(extension) => extension.span,
            TypeExtension::Union(extension) => extension.span,
            TypeExtension::Enum(extension) => extension.span,
            TypeExtension::InputObject(extension) => extension.span,
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#ScalarTypeExtension)
#[derive(Debug, PartialEq, Clone)]
pub struct ScalarTypeExtension {
    pub span: Span,
    pub name: Name,
    pub directives: Vec<Directive>,
}

/// [Spec](https://spec.graphql.org/October2021/#ObjectTypeExtension)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectTypeExtension {
    pub span: Span,
    pub name: Name,
    pub implement_interfaces: Vec<Name>,
    pub directives: Vec<Directive>,
    pub fields: Vec<FieldDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#InterfaceTypeExtension)
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeExtension {
    pub span: Span,
    pub name: Name,
    pub implement_interfaces: Vec<Name>,
    pub directives: Vec<Directive>,
    pub fields: Vec<FieldDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#UnionTypeExtension)
#[derive(Debug, PartialEq, Clone)]
pub struct UnionTypeExtension {
    pub span: Span,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub members: Vec<Name>,
}

/// [Spec](https://spec.graphql.org/October2021/#EnumTypeExtension)
#[derive(Debug, PartialEq, Clone)]
pub struct EnumTypeExtension {
    pub span: Span,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub values: Vec<EnumValueDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#InputObjectTypeExtension)
#[derive(Debug, PartialEq, Clone)]
pub struct InputObjectTypeExtension {
    pub span: Span,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub values: Vec<InputValueDefinition>,
}
