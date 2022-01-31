use crate::{GraphQLDescription, GraphQLDirective, GraphQLEnumValue};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphQLEnumType {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub directives: Vec<GraphQLDirective>,
    pub values: HashMap<String, GraphQLEnumValue>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

impl GraphQLEnumType {
    pub fn qualied_name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        self.description.as_ref()
    }
}
