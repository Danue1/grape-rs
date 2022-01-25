#![warn(clippy::all)]

use grape_span::Span;
use grape_symbol::Symbol;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Eof, // '<EOF>',

    /// !
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    Exclamation,

    /// $
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    Dollar,

    /// &
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    Ampersand,

    /// (
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    LeftParens,

    /// )
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    RightParens,

    /// ...
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    DotDotDot,

    /// :
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    Colon,

    /// =
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    Equal,

    /// @
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    At,

    /// [
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    LeftBracket,

    /// ]
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    RightBracket,

    /// {
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    LeftBrace,

    /// |
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    Pipe,

    /// }
    ///
    /// [Spec](https://spec.graphql.org/October2021/#sec-Punctuators)
    RightBrace,

    /// [Spec}(https://spec.graphql.org/October2021/#sec-Names)
    Name(Symbol),

    /// [Spec](https://spec.graphql.org/October2021/#IntValue)
    Integer(Symbol),

    /// [Spec](https://spec.graphql.org/October2021/#FloatValue)
    Float(Symbol),

    /// [Spec](https://spec.graphql.org/October2021/#StringValue)
    String(Symbol, bool),
}
