use crate::{GraphQLDescription, GraphQLDirective, GraphQLInputType, GraphQLValue};

#[derive(Debug)]
pub struct GraphQLInputValue {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub ty: GraphQLInputType,
    pub default_value: Option<GraphQLValue>,
    pub directives: Vec<GraphQLDirective>,
}
