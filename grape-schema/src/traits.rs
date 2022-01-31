use std::borrow::Cow;

pub trait GraphQLScalar: std::str::FromStr {
    const NAME: &'static str;

    fn description(&self) -> Option<Cow<str>>;
    fn serialize(&self) -> String;
}
