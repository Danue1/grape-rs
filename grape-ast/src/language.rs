use grape_span::Span;
use grape_symbol::Symbol;

/// [Spec](https://spec.graphql.org/October2021/#Name)
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
pub struct Name {
    pub span: Span,
    pub symbol: Symbol,
}

/// [Spec](https://spec.graphql.org/October2021/#Selection)
#[derive(Debug, PartialEq, Clone)]
pub enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
}

impl Selection {
    pub fn directives(&self) -> &[Directive] {
        match self {
            Selection::Field(field) => &field.directives,
            Selection::FragmentSpread(fragment_spread) => &fragment_spread.directives,
            Selection::InlineFragment(inline_fragment) => &inline_fragment.directives,
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#Field)
#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub span: Span,
    pub alias: Option<Name>,
    pub name: Name,
    pub arguments: Vec<Argument>,
    pub directives: Vec<Directive>,
    pub selections: Vec<Selection>,
}

/// [Spec](https://spec.graphql.org/October2021/#FragmentSpread)
#[derive(Debug, PartialEq, Clone)]
pub struct FragmentSpread {
    pub span: Span,
    pub name: Name,
    pub directives: Vec<Directive>,
}

/// [Spec](https://spec.graphql.org/October2021/#InlineFragment)
#[derive(Debug, PartialEq, Clone)]
pub struct InlineFragment {
    pub span: Span,
    pub name: Name,
    pub directives: Vec<Directive>,
    pub selections: Vec<Selection>,
}

/// [Spec](https://spec.graphql.org/October2021/#Argument)
#[derive(Debug, PartialEq, Clone)]
pub struct Argument {
    pub span: Span,
    pub key: Name,
    pub value: Value,
}

/// [Spec](https://spec.graphql.org/October2021/#Value)
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Variable(Variable),
    Int(IntValue),
    Float(FloatValue),
    String(StringValue),
    Boolean(BooleanValue),
    Null(NullValue),
    Enum(EnumValue),
    List(ListValue),
    Object(ObjectValue),
}

impl Value {
    pub const fn span(&self) -> Span {
        match self {
            Value::Variable(value) => value.span,
            Value::Int(value) => value.span,
            Value::Float(value) => value.span,
            Value::String(value) => value.span,
            Value::Boolean(value) => value.span,
            Value::Null(value) => value.span,
            Value::Enum(value) => value.span,
            Value::List(value) => value.span,
            Value::Object(value) => value.span,
        }
    }

    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(value) => Some(value.value),
            _ => None,
        }
    }
}

/// [Spec](https://spec.graphql.org/October2021/#Variable)
#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub span: Span,
    pub name: Name,
}

/// [Spec](https://spec.graphql.org/October2021/#IntValue)
#[derive(Debug, PartialEq, Clone)]
pub struct IntValue {
    pub span: Span,
    pub symbol: Symbol,
}

/// [Spec](https://spec.graphql.org/October2021/#FloatValue)
#[derive(Debug, PartialEq, Clone)]
pub struct FloatValue {
    pub span: Span,
    pub symbol: Symbol,
}

/// [Spec](https://spec.graphql.org/October2021/#StringValue)
#[derive(Debug, PartialEq, Clone)]
pub struct StringValue {
    pub span: Span,
    pub is_block: bool,
    pub symbol: Symbol,
}

/// [Spec](https://spec.graphql.org/October2021/#BooleanValue)
#[derive(Debug, PartialEq, Clone)]
pub struct BooleanValue {
    pub span: Span,
    pub value: bool,
}

/// [Spec](https://spec.graphql.org/October2021/#NullValue)
#[derive(Debug, PartialEq, Clone)]
pub struct NullValue {
    pub span: Span,
}

/// [Spec](https://spec.graphql.org/October2021/#EnumValue)
#[derive(Debug, PartialEq, Clone)]
pub struct EnumValue {
    pub span: Span,
    pub name: Name,
}

/// [Spec](https://spec.graphql.org/October2021/#ListValue)
#[derive(Debug, PartialEq, Clone)]
pub struct ListValue {
    pub span: Span,
    pub values: Vec<Value>,
}

/// [Spec](https://spec.graphql.org/October2021/#ObjectValue)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectValue {
    pub span: Span,
    pub fields: Vec<ObjectField>,
}

/// [Spec](https://spec.graphql.org/October2021/#ObjectField)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectField {
    pub span: Span,
    pub key: Name,
    pub value: Value,
}

/// [Spec](https://spec.graphql.org/October2021/#Type)
#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    pub span: Span,
    pub kind: TypeKind,
}

/// [Spec](https://spec.graphql.org/October2021/#Type)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    Named(Name),
    List(Box<Type>),
    NonNull(Box<Type>),
}

/// [Spec](https://spec.graphql.org/October2021/#Directive)
#[derive(Debug, PartialEq, Clone)]
pub struct Directive {
    pub span: Span,
    pub name: Name,
    pub arguments: Vec<Argument>,
}

impl Directive {
    pub fn argument(&self, key: &Symbol) -> Option<&Argument> {
        self.arguments
            .iter()
            .find(|argument| &argument.key.symbol == key)
    }
}
