use crate::GraphQLScalar;
use std::borrow::Cow;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct GraphQLIntValue {
    pub value: i64,
}

impl std::str::FromStr for GraphQLIntValue {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(|value| GraphQLIntValue { value })
    }
}

impl GraphQLScalar for GraphQLIntValue {
    const NAME: &'static str = "Int";

    fn description(&self) -> Option<Cow<str>> {
        None
    }

    fn serialize(&self) -> String {
        self.value.to_string()
    }
}

impl std::fmt::Display for GraphQLIntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
