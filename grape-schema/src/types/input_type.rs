use crate::{GraphQLDescription, GraphQLNamedInputType};
use std::borrow::Cow;

#[derive(Debug)]
pub enum GraphQLInputType {
    Named(GraphQLNamedInputType),
    List(Box<GraphQLInputType>),
    NonNull(Box<GraphQLInputType>),
}

impl GraphQLInputType {
    pub fn name(&self) -> &str {
        match self {
            GraphQLInputType::Named(ty) => ty.name(),
            GraphQLInputType::List(ty) => ty.name(),
            GraphQLInputType::NonNull(ty) => ty.name(),
        }
    }

    pub fn qualied_name(&self) -> Cow<str> {
        match self {
            GraphQLInputType::Named(ty) => Cow::Borrowed(ty.qualied_name()),
            GraphQLInputType::List(ty) => Cow::Owned(format!("[{}]", ty.qualied_name())),
            GraphQLInputType::NonNull(ty) => Cow::Owned(format!("{}!", ty.qualied_name())),
        }
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        match self {
            GraphQLInputType::Named(ty) => ty.description(),
            GraphQLInputType::List(ty) => ty.description(),
            GraphQLInputType::NonNull(ty) => ty.description(),
        }
    }
}
