use crate::{GraphQLDescription, GraphQLDirective};

#[derive(Debug)]
pub struct GraphQLScalarType {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub directives: Vec<GraphQLDirective>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

impl GraphQLScalarType {
    pub fn qualied_name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&GraphQLDescription> {
        self.description.as_ref()
    }
}
