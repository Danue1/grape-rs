use crate::{GraphQLDirective, GraphQLNamedType, GraphQLObjectType};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct GraphQLSchema<'schema> {
    pub query: GraphQLObjectType,
    pub mutation: Option<GraphQLObjectType>,
    pub subscription: Option<GraphQLObjectType>,
    pub directives: HashMap<String, GraphQLDirective>,
    pub types: HashMap<String, &'schema GraphQLNamedType>,
    pub sub_types: HashMap<String, HashSet<String>>,
    pub implementation_objects: HashMap<String, HashSet<String>>,
    pub implementation_interfaces: HashMap<String, HashSet<String>>,
}

impl<'schema> GraphQLSchema<'schema> {
    pub fn directive(&self, name: &str) -> Option<&GraphQLDirective> {
        self.directives.get(name)
    }

    pub fn ty(&self, name: &str) -> Option<&GraphQLNamedType> {
        match self.types.get(name) {
            Some(ty) => Some(ty),
            None => None,
        }
    }

    pub fn sub_type(&self, name: &str) -> Option<&HashSet<String>> {
        match self.sub_types.get(name) {
            Some(sub_types) => Some(sub_types),
            None => None,
        }
    }

    pub fn implementation_object(&self, name: &str) -> Option<&HashSet<String>> {
        match self.implementation_objects.get(name) {
            Some(implementation_objects) => Some(implementation_objects),
            None => None,
        }
    }

    pub fn implementation_interface(&self, name: &str) -> Option<&HashSet<String>> {
        match self.implementation_interfaces.get(name) {
            Some(implementation_interfaces) => Some(implementation_interfaces),
            None => None,
        }
    }
}
