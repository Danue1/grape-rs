use crate::GraphQLScalar;
use std::borrow::Cow;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct GraphQLStringValue {
    pub value: String,
}

impl std::str::FromStr for GraphQLStringValue {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(GraphQLStringValue {
            value: value.to_owned(),
        })
    }
}

impl GraphQLScalar for GraphQLStringValue {
    const NAME: &'static str = "String";

    fn description(&self) -> Option<Cow<str>> {
        None
    }

    fn serialize(&self) -> String {
        self.value.to_owned()
    }
}

impl std::fmt::Display for GraphQLStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
