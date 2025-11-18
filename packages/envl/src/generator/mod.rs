use std::io::Error;

use crate::{
    generator::{js::generate_js_file, rust::generate_rust_file, ts::generate_ts_file},
    VariableHashMap,
};

pub mod js;
pub mod rust;
pub mod ts;

#[derive(Debug, Clone, Default)]
pub struct GenerateOptions {
    pub language: Option<String>,
    pub cjs: Option<bool>,
}

pub fn generate_file(
    data: VariableHashMap,
    output: String,
    options: GenerateOptions,
) -> Result<String, Error> {
    if options.language.clone().is_some_and(|l| l == "ts")
        || output.ends_with(".ts")
        || output.ends_with(".mts")
        || output.ends_with(".cts")
    {
        generate_ts_file(data, options)
    } else if options.language.clone().is_some_and(|l| l == "js")
        || output.ends_with(".js")
        || output.ends_with(".mjs")
        || output.ends_with(".cjs")
    {
        generate_js_file(data, options)
    } else if options.language.is_some_and(|l| l == "rust") || output.ends_with(".rs") {
        generate_rust_file(data)
    } else {
        Err(Error::other("Unsupported file"))
    }
}
