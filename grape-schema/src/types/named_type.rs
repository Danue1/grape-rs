use crate::{
    GraphQLDescription, GraphQLEnumType, GraphQLInputObjectType, GraphQLInterfaceType,
    GraphQLObjectType, GraphQLScalarType, GraphQLUnionType,
};

#[derive(Debug)]
pub enum GraphQLNamedType {
    Scalar(GraphQLScalarType),
    Object(GraphQLObjectType),
    Interface(GraphQLInterfaceType),
    Union(GraphQLUnionType),
    Enum(GraphQLEnumType),
    InputObject(GraphQLInputObjectType),
}

impl GraphQLNamedType {
    pub fn name(&self) -> &str {
        match self {
            GraphQLNamedType::Scalar(ty) => &ty.name,
            GraphQLNamedType::Object(ty) => &ty.name,
            GraphQLNamedType::Interface(ty) => &ty.name,
            GraphQLNamedType::Union(ty) => &ty.name,
            GraphQLNamedType::Enum(ty) => &ty.name,
            GraphQLNamedType::InputObject(ty) => &ty.name,
        }
    }

    pub fn qualied_name(&self) -> &str {
        match self {
            GraphQLNamedType::Scalar(ty) => ty.qualied_name(),
            GraphQLNamedType::Object(ty) => ty.qualied_name(),
            GraphQLNamedType::Interface(ty) => ty.qualied_name(),
            GraphQLNamedType::Union(ty) => ty.qualied_name(),
            GraphQLNamedType::Enum(ty) => ty.qualied_name(),
            GraphQLNamedType::InputObject(ty) => ty.qualied_name(),
        }
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        match self {
            GraphQLNamedType::Scalar(ty) => ty.description(),
            GraphQLNamedType::Object(ty) => ty.description(),
            GraphQLNamedType::Interface(ty) => ty.description(),
            GraphQLNamedType::Union(ty) => ty.description(),
            GraphQLNamedType::Enum(ty) => ty.description(),
            GraphQLNamedType::InputObject(ty) => ty.description(),
        }
    }
}
