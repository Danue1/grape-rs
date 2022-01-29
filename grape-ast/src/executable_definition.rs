use crate::{Directive, Name, Selection, Type, Value};
use grape_span::Span;

/// [Spec](https://spec.graphql.org/October2021/#OperationDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct OperationDefinition {
    pub span: Span,
    pub operation: OperationType,
    pub name: Option<Name>,
    pub variables: Vec<VariableDefinition>,
    pub directives: Vec<Directive>,
    pub selections: Vec<Selection>,
}

/// [Spec](https://spec.graphql.org/October2021/#OperationType)
#[derive(Debug, PartialEq, Clone)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

/// [Spec](https://spec.graphql.org/October2021/#VariableDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct VariableDefinition {
    pub span: Span,
    pub name: Name,
    pub ty: Type,
    pub default_value: Option<Value>,
    pub directives: Vec<Directive>,
}

/// [Spec](https://spec.graphql.org/October2021/#FragmentDefinition)
#[derive(Debug, PartialEq, Clone)]
pub struct FragmentDefinition {
    pub span: Span,
    pub name: Name,
    pub type_condition: Name,
    pub directives: Vec<Directive>,
    pub selections: Vec<Selection>,
}
