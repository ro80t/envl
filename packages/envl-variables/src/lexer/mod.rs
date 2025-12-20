use envl_utils::{
    curr_token::{CurrentToken, Push},
    types::{FilePosition, Position},
};

use crate::misc::token::{Token, Value};

pub mod current_token;
pub mod test;

pub(crate) struct Lexer {
    file_path: String,
    code: String,
}

impl Lexer {
    pub fn new(file_path: String, code: String) -> Self {
        Self { file_path, code }
    }

    pub fn generate(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut row = 1;
        let mut col = 1;
        let mut in_quote = false;
        let mut is_comment = false;
        let mut is_escape = false;
        let mut start_quote = char::default();
        let mut current_token = CurrentToken::default();

        'lexer_loop: for (i, c) in self.code.char_indices() {
            let is_last = self.code.len() == (i + 1);
            let mut is_others = false;
            let mut file_pos = FilePosition { row, col };
            let mut curr_token_pos = Position {
                file_path: self.file_path.clone(),
                start: current_token.start.clone(),
                end: file_pos.clone(),
            };

            if is_comment && (c == '\n' || is_last) {
                if c != '\n' && is_last {
                    current_token.push(c, file_pos);
                }
                tokens.push(Token {
                    value: Value::Comment(current_token.token.clone().chars().skip(2).collect()),
                    position: curr_token_pos,
                });
                current_token.clear();
                is_comment = false;
            }

            if c == '\n' {
                row += 1;
                col = 1;
                continue;
            }

            file_pos = FilePosition { row, col };
            curr_token_pos = Position {
                file_path: self.file_path.clone(),
                start: current_token.start.clone(),
                end: if c.is_whitespace() {
                    FilePosition { row, col: col - 1 }
                } else {
                    file_pos.clone()
                },
            };

            let position = Position {
                file_path: self.file_path.clone(),
                start: file_pos.clone(),
                end: file_pos.clone(),
            };

            if is_escape {
                current_token.push(
                    match c {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '0' => '\0',
                        _ => c,
                    },
                    file_pos,
                );
                is_escape = false;
                col += 1;
                continue;
            }

            if (in_quote && c != start_quote) || is_comment {
                current_token.push(c, file_pos);
                col += 1;
                continue;
            }

            match c {
                '"' | '\'' => {
                    if in_quote {
                        tokens.push(Token {
                            value: Value::Ident(format!(
                                "{}{}{}",
                                start_quote,
                                current_token.token.clone(),
                                c
                            )),
                            position: Position {
                                start: FilePosition {
                                    col: current_token.start.col - 1,
                                    ..current_token.start
                                },
                                ..curr_token_pos.clone()
                            },
                        });
                        start_quote = char::default();
                    } else {
                        start_quote = c;
                    }
                    in_quote = !in_quote;
                    current_token.clear();
                }
                '\\' => {
                    is_escape = true;
                }
                '[' => {
                    tokens.push(Token {
                        value: Value::LeftSquareBracket,
                        position: position.clone(),
                    });
                }
                ']' => {
                    tokens.push(Token {
                        value: Value::RightSquareBracket,
                        position: position.clone(),
                    });
                }
                '{' => {
                    tokens.push(Token {
                        value: Value::LeftCurlyBracket,
                        position: position.clone(),
                    });
                }
                '}' => {
                    tokens.push(Token {
                        value: Value::RightCurlyBracket,
                        position: position.clone(),
                    });
                }
                ':' => {
                    tokens.push(Token {
                        value: Value::Colon,
                        position: position.clone(),
                    });
                }
                ',' => {
                    tokens.push(Token {
                        value: Value::Comma,
                        position: position.clone(),
                    });
                }
                ';' => {
                    tokens.push(Token {
                        value: Value::Semi,
                        position: position.clone(),
                    });
                }
                '=' => {
                    tokens.push(Token {
                        value: Value::Equal,
                        position: position.clone(),
                    });
                }
                '/' => {
                    current_token.push(c, file_pos.clone());
                    if current_token.token == "/" {
                        is_comment = true;
                    } else {
                        continue 'lexer_loop;
                    }
                }
                other => {
                    if other.is_whitespace() && !in_quote && !is_comment {
                        if !current_token.is_empty() {
                            let identifier = self.lex_current_token(current_token.token.clone());
                            tokens.push(Token {
                                value: identifier,
                                position: curr_token_pos.clone(),
                            });
                            current_token.clear();
                        }
                    } else {
                        current_token.push(c, file_pos);
                    }
                    is_others = true;
                }
            }

            if !is_comment && !in_quote && !is_others && !current_token.is_empty() {
                let identifier = self.lex_current_token(current_token.token.clone());
                tokens.insert(
                    tokens.len() - 1,
                    Token {
                        value: identifier,
                        position: if is_others {
                            curr_token_pos
                        } else {
                            Position {
                                end: FilePosition { row, col: col - 1 },
                                ..curr_token_pos
                            }
                        },
                    },
                );
                current_token.clear();
            }

            col += 1;
        }

        tokens
    }
}
