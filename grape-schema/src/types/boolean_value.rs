use crate::GraphQLScalar;
use std::borrow::Cow;
use std::str::ParseBoolError;

#[derive(Debug)]
pub struct GraphQLBooleanValue {
    pub value: bool,
}

impl std::str::FromStr for GraphQLBooleanValue {
    type Err = ParseBoolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(|value| GraphQLBooleanValue { value })
    }
}

impl GraphQLScalar for GraphQLBooleanValue {
    const NAME: &'static str = "Boolean";

    fn description(&self) -> Option<Cow<str>> {
        None
    }

    fn serialize(&self) -> String {
        self.value.to_string()
    }
}

impl std::fmt::Display for GraphQLBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
