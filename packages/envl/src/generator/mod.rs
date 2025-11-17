use std::io::Error;

use crate::{generator::rust::generate_rust_file, VariableHashMap};

pub mod rust;

#[derive(Debug, Clone, Default)]
pub struct GenerateOptions {
    pub language: Option<String>,
}

pub fn generate_file(
    data: VariableHashMap,
    output: String,
    options: GenerateOptions,
) -> Result<String, Error> {
    if options.language.is_some_and(|l| l == "rust") || output.ends_with(".rs") {
        generate_rust_file(data)
    } else {
        Err(Error::other("Unsupported file"))
    }
}
