use crate::{GraphQLDirective, GraphQLNamedType, GraphQLObjectType, GraphQLSchema};
use grape_diagnostics::Diagnostics;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct GraphQLSchemaBuilder<'schema> {
    pub query: GraphQLObjectType,
    pub mutation: Option<GraphQLObjectType>,
    pub subscription: Option<GraphQLObjectType>,
    pub directives: HashMap<String, GraphQLDirective>,
    pub types: HashMap<String, &'schema GraphQLNamedType>,
    pub sub_types: HashMap<String, HashSet<String>>,
    pub implementation_objects: HashMap<String, HashSet<String>>,
    pub implementation_interfaces: HashMap<String, HashSet<String>>,
}

impl<'schema> GraphQLSchemaBuilder<'schema> {
    pub fn new(query: GraphQLObjectType) -> Self {
        GraphQLSchemaBuilder {
            query,
            mutation: None,
            subscription: None,
            directives: HashMap::new(),
            types: HashMap::new(),
            sub_types: HashMap::new(),
            implementation_objects: HashMap::new(),
            implementation_interfaces: HashMap::new(),
        }
    }

    pub fn mutation(mut self, mutation: GraphQLObjectType) -> Self {
        self.mutation = Some(mutation);
        self
    }

    pub fn subscription(mut self, subscription: GraphQLObjectType) -> Self {
        self.subscription = Some(subscription);
        self
    }

    pub fn directives(mut self, directives: HashMap<String, GraphQLDirective>) -> Self {
        for (key, directive) in directives {
            self.directives.insert(key, directive);
        }
        self
    }

    pub fn directive(mut self, key: &str, directive: GraphQLDirective) -> Self {
        self.directives.insert(key.to_owned(), directive);
        self
    }

    pub fn types(mut self, types: HashMap<String, &'schema GraphQLNamedType>) -> Self {
        for (name, type_) in types {
            self.types.insert(name, type_);
        }
        self
    }

    pub fn ty(mut self, name: &str, ty: &'schema GraphQLNamedType) -> Self {
        self.types.insert(name.to_owned(), ty);
        self
    }

    pub fn sub_types(mut self, sub_types: HashMap<String, HashSet<String>>) -> Self {
        for (key, value) in sub_types {
            self.sub_types.insert(key, value);
        }
        self
    }

    pub fn sub_type(mut self, key: &str, value: HashSet<String>) -> Self {
        self.sub_types.insert(key.to_owned(), value);
        self
    }

    pub fn implementation_objects(
        mut self,
        implementation_objects: HashMap<String, HashSet<String>>,
    ) -> Self {
        for (key, value) in implementation_objects {
            self.implementation_objects.insert(key, value);
        }
        self
    }

    pub fn implementation_object(mut self, key: &str, value: HashSet<String>) -> Self {
        self.implementation_objects.insert(key.to_owned(), value);
        self
    }

    pub fn implementation_interfaces(
        mut self,
        implementation_interfaces: HashMap<String, HashSet<String>>,
    ) -> Self {
        for (key, value) in implementation_interfaces {
            self.implementation_interfaces.insert(key, value);
        }
        self
    }

    pub fn implementation_interface(mut self, key: &str, value: HashSet<String>) -> Self {
        self.implementation_interfaces.insert(key.to_owned(), value);
        self
    }

    pub fn build(self) -> Result<GraphQLSchema<'schema>, Diagnostics<'schema>> {
        // let mut reference_types = HashMap::new();
        // collect_reference_types(&self.query, &mut reference_types);

        Ok(GraphQLSchema {
            query: self.query,
            mutation: self.mutation,
            subscription: self.subscription,
            directives: self.directives,
            types: self.types,
            sub_types: self.sub_types,
            implementation_objects: self.implementation_objects,
            implementation_interfaces: self.implementation_interfaces,
        })
    }
}
