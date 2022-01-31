#[derive(Debug)]
pub struct GraphQLDescription {
    pub is_block: bool,
    pub value: String,
}

impl std::fmt::Display for GraphQLDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_block {
            write!(f, "\"\"\"{}\"\"\"", self.value)
        } else {
            write!(f, "\"{}\"", self.value)
        }
    }
}
