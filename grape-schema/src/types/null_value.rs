use crate::GraphQLScalar;
use std::borrow::Cow;

#[derive(Debug)]
pub struct GraphQLNullValue;

impl std::str::FromStr for GraphQLNullValue {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "null" {
            Ok(GraphQLNullValue)
        } else {
            Err(())
        }
    }
}

impl GraphQLScalar for GraphQLNullValue {
    const NAME: &'static str = "Null";

    fn description(&self) -> Option<Cow<str>> {
        None
    }

    fn serialize(&self) -> String {
        "null".to_owned()
    }
}

impl std::fmt::Display for GraphQLNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "null")
    }
}
