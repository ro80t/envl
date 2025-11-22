use envl_utils::{types::Position, variable::Type};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Value {
    Comment(String),
    Ident(String),
    Type(Type),
    RightSquareBracket,
    LeftSquareBracket,
    RightCurlyBracket,
    LeftCurlyBracket,
    RightParentheses,
    LeftParentheses,
    RightShift,
    LeftShift,
    Settings,
    Struct,
    Option,
    Array,
    Comma,
    Colon,
    Equal,
    Null,
    Vars,
    Semi,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    pub value: Value,
    pub position: Position,
}
