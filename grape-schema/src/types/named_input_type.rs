use crate::{GraphQLDescription, GraphQLEnumType, GraphQLInputObjectType, GraphQLScalarType};

#[derive(Debug)]
pub enum GraphQLNamedInputType {
    Scalar(GraphQLScalarType),
    Enum(GraphQLEnumType),
    InputObject(GraphQLInputObjectType),
}

impl GraphQLNamedInputType {
    pub fn name(&self) -> &str {
        match self {
            GraphQLNamedInputType::Scalar(ty) => &ty.name,
            GraphQLNamedInputType::Enum(ty) => &ty.name,
            GraphQLNamedInputType::InputObject(ty) => &ty.name,
        }
    }

    pub fn qualied_name(&self) -> &str {
        match self {
            GraphQLNamedInputType::Scalar(ty) => ty.qualied_name(),
            GraphQLNamedInputType::Enum(ty) => ty.qualied_name(),
            GraphQLNamedInputType::InputObject(ty) => ty.qualied_name(),
        }
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        match self {
            GraphQLNamedInputType::Scalar(ty) => ty.description(),
            GraphQLNamedInputType::Enum(ty) => ty.description(),
            GraphQLNamedInputType::InputObject(ty) => ty.description(),
        }
    }
}
