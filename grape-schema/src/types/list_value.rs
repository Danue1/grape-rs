use crate::GraphQLValue;

#[derive(Debug)]
pub struct GraphQLListValue {
    pub values: Vec<GraphQLValue>,
}

impl std::fmt::Display for GraphQLListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;

        let mut values = self.values.iter();
        if let Some(value) = values.next() {
            write!(f, "{}", value)?;
        }
        for value in values {
            write!(f, ", {}", value)?;
        }

        write!(f, "]")
    }
}
