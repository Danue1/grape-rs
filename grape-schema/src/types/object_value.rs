use crate::GraphQLValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphQLObjectValue {
    pub fields: HashMap<String, GraphQLValue>,
}

impl std::fmt::Display for GraphQLObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;

        let mut fields = self.fields.iter();
        if let Some((key, value)) = fields.next() {
            write!(f, "{}: {}", key, value)?;
        }
        for (key, value) in fields {
            write!(f, ", {}: {}", key, value)?;
        }

        write!(f, "}}")
    }
}
