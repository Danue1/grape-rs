#[derive(Debug)]
pub enum GraphQLDirectiveLocation {
    // Executable directives.
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
    VariableDefinition,

    // Type system directives.
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}

impl std::fmt::Display for GraphQLDirectiveLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Executable directives.
            GraphQLDirectiveLocation::Query => write!(f, "QUERY"),
            GraphQLDirectiveLocation::Mutation => write!(f, "MUTATION"),
            GraphQLDirectiveLocation::Subscription => write!(f, "SUBSCRIPTION"),
            GraphQLDirectiveLocation::Field => write!(f, "FIELD"),
            GraphQLDirectiveLocation::FragmentDefinition => write!(f, "FRAGMENT_DEFINITION"),
            GraphQLDirectiveLocation::FragmentSpread => write!(f, "FRAGMENT_SPREAD"),
            GraphQLDirectiveLocation::InlineFragment => write!(f, "INLINE_FRAGMENT"),
            GraphQLDirectiveLocation::VariableDefinition => write!(f, "VARIABLE_DEFINITION"),

            // Type system directives.
            GraphQLDirectiveLocation::Schema => write!(f, "SCHEMA"),
            GraphQLDirectiveLocation::Scalar => write!(f, "SCALAR"),
            GraphQLDirectiveLocation::Object => write!(f, "OBJECT"),
            GraphQLDirectiveLocation::FieldDefinition => write!(f, "FIELD_DEFINITION"),
            GraphQLDirectiveLocation::ArgumentDefinition => write!(f, "ARGUMENT_DEFINITION"),
            GraphQLDirectiveLocation::Interface => write!(f, "INTERFACE"),
            GraphQLDirectiveLocation::Union => write!(f, "UNION"),
            GraphQLDirectiveLocation::Enum => write!(f, "ENUM"),
            GraphQLDirectiveLocation::EnumValue => write!(f, "ENUM_VALUE"),
            GraphQLDirectiveLocation::InputObject => write!(f, "INPUT_OBJECT"),
            GraphQLDirectiveLocation::InputFieldDefinition => write!(f, "INPUT_FIELD_DEFINITION"),
        }
    }
}
