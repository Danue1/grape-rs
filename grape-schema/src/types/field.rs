use crate::{GraphQLDescription, GraphQLDirective, GraphQLInputValue, GraphQLOutputType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphQLField {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub args: HashMap<String, GraphQLInputValue>,
    pub ty: GraphQLOutputType,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
    pub directives: Vec<GraphQLDirective>,
}
