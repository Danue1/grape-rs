use crate::{GraphQLDescription, GraphQLDirective, GraphQLInputType};

#[derive(Debug)]
pub struct GraphQLInputField {
    pub name: String,
    pub description: Option<GraphQLDescription>,
    pub directives: Vec<GraphQLDirective>,
    pub ty: GraphQLInputType,
}
