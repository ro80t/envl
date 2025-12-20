use envl_utils::{
    error::EnvlError,
    types::{FilePosition, Position},
};

use crate::{lexer::Lexer, misc::config::Config, parser::Parser};

pub mod lexer;
pub mod misc;
pub mod parser;
pub mod test;

pub fn generate_ast(file_path: String, code: String) -> Result<Config, EnvlError> {
    let lex = Lexer::new(file_path.clone(), code);
    let (tokens, file_end_pos) = lex.generate();
    let parser = Parser::new(
        Position {
            file_path,
            start: FilePosition { col: 1, row: 1 },
            end: file_end_pos,
        },
        tokens,
    );
    parser.parse()
}
