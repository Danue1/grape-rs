use crate::{GraphQLDescription, GraphQLDirective, GraphQLInputField};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphQLInputObjectType {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub directives: Vec<GraphQLDirective>,
    pub fields: HashMap<String, GraphQLInputField>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

impl GraphQLInputObjectType {
    pub fn qualied_name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        self.description.as_ref()
    }
}
