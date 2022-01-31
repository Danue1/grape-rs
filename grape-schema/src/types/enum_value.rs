use crate::{GraphQLDescription, GraphQLDirective};

#[derive(Debug)]
pub struct GraphQLEnumValue {
    pub description: Option<GraphQLDescription>,
    pub name: String,
    pub directives: Vec<GraphQLDirective>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

impl std::fmt::Display for GraphQLEnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
