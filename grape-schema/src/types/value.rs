use crate::{
    GraphQLBooleanValue, GraphQLEnumValue, GraphQLFloatValue, GraphQLIntValue, GraphQLListValue,
    GraphQLNullValue, GraphQLObjectValue, GraphQLStringValue,
};

#[derive(Debug)]
pub enum GraphQLValue {
    Null(GraphQLNullValue),
    Int(GraphQLIntValue),
    Float(GraphQLFloatValue),
    String(GraphQLStringValue),
    Boolean(GraphQLBooleanValue),
    Enum(GraphQLEnumValue),
    List(GraphQLListValue),
    Object(GraphQLObjectValue),
}

impl std::fmt::Display for GraphQLValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphQLValue::Null(value) => write!(f, "{}", value),
            GraphQLValue::Int(value) => write!(f, "{}", value),
            GraphQLValue::Float(value) => write!(f, "{}", value),
            GraphQLValue::String(value) => write!(f, "{}", value),
            GraphQLValue::Boolean(value) => write!(f, "{}", value),
            GraphQLValue::Enum(value) => write!(f, "{}", value),
            GraphQLValue::List(value) => write!(f, "{}", value),
            GraphQLValue::Object(value) => write!(f, "{}", value),
        }
    }
}
