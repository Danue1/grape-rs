use crate::{GraphQLDescription, GraphQLDirective, GraphQLField, GraphQLInterfaceType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphQLObjectType {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub interfaces: Vec<GraphQLInterfaceType>,
    pub fields: HashMap<String, GraphQLField>,
    pub directives: Vec<GraphQLDirective>,
}

impl GraphQLObjectType {
    pub fn qualied_name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        self.description.as_ref()
    }
}
