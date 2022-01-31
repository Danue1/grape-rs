use crate::{GraphQLDescription, GraphQLDirective, GraphQLNamedType};
use std::rc::Rc;

#[derive(Debug)]
pub struct GraphQLUnionType {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub directives: Vec<GraphQLDirective>,
    pub types: Vec<Rc<GraphQLNamedType>>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

impl GraphQLUnionType {
    pub fn qualied_name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        self.description.as_ref()
    }
}
