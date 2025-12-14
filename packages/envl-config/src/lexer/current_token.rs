use envl_utils::{types::FilePosition, variable::Type};

use crate::{lexer::Lexer, misc::token::Value};

pub(crate) trait Push<T> {
    fn push(&mut self, c: T, pos: FilePosition);
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CurrentToken {
    pub token: String,
    pub start: FilePosition,
}

impl CurrentToken {
    pub fn clear(&mut self) {
        self.token.clear();
        self.start = FilePosition::default();
    }

    pub fn is_empty(&mut self) -> bool {
        self.token.is_empty()
    }
}

impl Push<char> for CurrentToken {
    fn push(&mut self, c: char, pos: FilePosition) {
        if self.token.is_empty() {
            self.start = pos;
        }
        self.token.push(c);
    }
}

impl Push<String> for CurrentToken {
    fn push(&mut self, c: String, pos: FilePosition) {
        if self.token.is_empty() {
            self.start = pos;
        }
        self.token.push_str(&c);
    }
}

impl Lexer {
    pub(super) fn lex_current_token(&self, current_token: String) -> Value {
        match current_token.as_str() {
            "string" => Value::Type(Type::String),
            "char" => Value::Type(Type::Char),
            "int" => Value::Type(Type::Int),
            "uint" => Value::Type(Type::Uint),
            "bool" => Value::Type(Type::Bool),
            "float" => Value::Type(Type::Float),
            "settings" => Value::Settings,
            "vars" => Value::Vars,
            "struct" => Value::Struct,
            "Array" => Value::Array,
            "Option" => Value::Option,
            "null" => Value::Null,
            other => Value::Ident(other.to_string()),
        }
    }
}
