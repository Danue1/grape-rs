use crate::{GraphQLDescription, GraphQLDirective, GraphQLField};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphQLInterfaceType {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub fields: HashMap<String, GraphQLField>,
    pub directives: Vec<GraphQLDirective>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

impl GraphQLInterfaceType {
    pub fn qualied_name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        self.description.as_ref()
    }
}
