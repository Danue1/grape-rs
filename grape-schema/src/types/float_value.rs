use crate::GraphQLScalar;
use std::borrow::Cow;
use std::num::ParseFloatError;

#[derive(Debug)]
pub struct GraphQLFloatValue {
    pub value: f64,
}

impl std::str::FromStr for GraphQLFloatValue {
    type Err = ParseFloatError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(|value| GraphQLFloatValue { value })
    }
}

impl GraphQLScalar for GraphQLFloatValue {
    const NAME: &'static str = "Float";

    fn description(&self) -> Option<Cow<str>> {
        None
    }

    fn serialize(&self) -> String {
        self.value.to_string()
    }
}

impl std::fmt::Display for GraphQLFloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
