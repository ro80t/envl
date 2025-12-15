use envl_utils::types::Position;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Value {
    Comment(String),
    Ident(String),
    RightSquareBracket,
    LeftSquareBracket,
    RightCurlyBracket,
    LeftCurlyBracket,
    Struct,
    Comma,
    Colon,
    Equal,
    Semi,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    pub value: Value,
    pub position: Position,
}
