use crate::{FragmentDefinition, OperationDefinition, TypeSystemDefinition, TypeSystemExtension};
use grape_span::Span;

/// [Spec](https://spec.graphql.org/October2021/#Document)
#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub span: Span,
    pub definitions: Vec<Definition>,
}

/// [Spec](https://spec.graphql.org/October2021/#TypeSystemDocument)
#[derive(Debug, PartialEq, Clone)]
pub struct TypeSystemDocument {
    pub span: Span,
    pub definitions: Vec<TypeSystemDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#TypeSystemExtensionDocument)
#[derive(Debug, PartialEq, Clone)]
pub struct TypeSystemExtensionDocument {
    pub span: Span,
    pub definitions: Vec<TypeSystemDefinitionOrExtension>,
}

/// [Spec](https://spec.graphql.org/October2021/#ExecutableDocument)
#[derive(Debug, PartialEq, Clone)]
pub struct ExecutableDocument {
    pub span: Span,
    pub definitions: Vec<ExecutableDefinition>,
}

/// [Spec](https://spec.graphql.org/October2021/#Definition)
#[derive(Debug, PartialEq, Clone)]
pub enum Definition {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
    TypeSystem(TypeSystemDefinition),
    Extension(TypeSystemExtension),
}

impl Definition {
    pub const fn span(&self) -> Span {
        match self {
            Definition::Operation(definition) => definition.span,
            Definition::Fragment(definition) => definition.span,
            Definition::TypeSystem(definition) => definition.span(),
            Definition::Extension(definition) => definition.span(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeSystemDefinitionOrExtension {
    Definition(TypeSystemDefinition),
    Extension(TypeSystemExtension),
}

impl TypeSystemDefinitionOrExtension {
    pub const fn span(&self) -> Span {
        match self {
            TypeSystemDefinitionOrExtension::Definition(definition) => definition.span(),
            TypeSystemDefinitionOrExtension::Extension(definition) => definition.span(),
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#ExecutableDefinition)
#[derive(Debug, PartialEq, Clone)]
pub enum ExecutableDefinition {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
}

impl ExecutableDefinition {
    pub const fn span(&self) -> Span {
        match self {
            ExecutableDefinition::Operation(definition) => definition.span,
            ExecutableDefinition::Fragment(definition) => definition.span,
        }
    }
}
