use crate::{GraphQLDescription, GraphQLNamedOutputType};
use std::borrow::Cow;

#[derive(Debug)]
pub enum GraphQLOutputType {
    Named(GraphQLNamedOutputType),
    List(Box<GraphQLOutputType>),
    NonNull(Box<GraphQLOutputType>),
}

impl GraphQLOutputType {
    pub fn name(&self) -> &str {
        match self {
            GraphQLOutputType::Named(ty) => ty.name(),
            GraphQLOutputType::List(ty) => ty.name(),
            GraphQLOutputType::NonNull(ty) => ty.name(),
        }
    }

    pub fn qualied_name(&self) -> Cow<str> {
        match self {
            GraphQLOutputType::Named(ty) => Cow::Borrowed(ty.qualied_name()),
            GraphQLOutputType::List(ty) => Cow::Owned(format!("[{}]", ty.qualied_name())),
            GraphQLOutputType::NonNull(ty) => Cow::Owned(format!("{}!", ty.qualied_name())),
        }
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        match self {
            GraphQLOutputType::Named(ty) => ty.description(),
            GraphQLOutputType::List(ty) => ty.description(),
            GraphQLOutputType::NonNull(ty) => ty.description(),
        }
    }
}
