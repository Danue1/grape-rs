use crate::{
    GraphQLDescription, GraphQLEnumType, GraphQLInterfaceType, GraphQLObjectType,
    GraphQLScalarType, GraphQLUnionType,
};

#[derive(Debug)]
pub enum GraphQLNamedOutputType {
    Scalar(GraphQLScalarType),
    Object(GraphQLObjectType),
    Interface(GraphQLInterfaceType),
    Union(GraphQLUnionType),
    Enum(GraphQLEnumType),
}

impl GraphQLNamedOutputType {
    pub fn name(&self) -> &str {
        match self {
            GraphQLNamedOutputType::Scalar(ty) => &ty.name,
            GraphQLNamedOutputType::Object(ty) => &ty.name,
            GraphQLNamedOutputType::Interface(ty) => &ty.name,
            GraphQLNamedOutputType::Union(ty) => &ty.name,
            GraphQLNamedOutputType::Enum(ty) => &ty.name,
        }
    }

    pub fn qualied_name(&self) -> &str {
        match self {
            GraphQLNamedOutputType::Scalar(ty) => ty.qualied_name(),
            GraphQLNamedOutputType::Object(ty) => ty.qualied_name(),
            GraphQLNamedOutputType::Interface(ty) => ty.qualied_name(),
            GraphQLNamedOutputType::Union(ty) => ty.qualied_name(),
            GraphQLNamedOutputType::Enum(ty) => ty.qualied_name(),
        }
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        match self {
            GraphQLNamedOutputType::Scalar(ty) => ty.description(),
            GraphQLNamedOutputType::Object(ty) => ty.description(),
            GraphQLNamedOutputType::Interface(ty) => ty.description(),
            GraphQLNamedOutputType::Union(ty) => ty.description(),
            GraphQLNamedOutputType::Enum(ty) => ty.description(),
        }
    }
}
