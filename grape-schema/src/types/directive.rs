use crate::{GraphQLDescription, GraphQLDirectiveLocation, GraphQLInputValue};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphQLDirective {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub locations: Vec<GraphQLDirectiveLocation>,
    pub args: HashMap<String, GraphQLInputValue>,
    pub is_repeatable: bool,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}
