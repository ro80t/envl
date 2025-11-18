use std::io::Error;

use envl_codeblock::code_block;
use proc_macro2::TokenStream;

use crate::{
    generator::{js::value::gen_value, GenerateOptions},
    VariableHashMap,
};

pub mod gen_array;
pub mod gen_struct;
pub mod value;

pub fn generate_js_file(data: VariableHashMap, options: GenerateOptions) -> Result<String, Error> {
    match generate_js_file_core(&data) {
        Ok(v) => {
            let base_code = code_block! {
                const envl = #v;
            };
            if options.cjs.is_some_and(|cjs| cjs) {
                Ok(code_block! {
                    "use strict";
                    #base_code
                    module.exports = { envl }
                }
                .to_string())
            } else {
                Ok(code_block! {
                    #base_code
                    export { envl };
                }
                .to_string())
            }
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn generate_js_file_core(data: &VariableHashMap) -> Result<TokenStream, Error> {
    let mut vars = Vec::new();

    for (n, v) in data {
        let name = n.parse::<TokenStream>().unwrap();
        match gen_value(v.value.clone()) {
            Ok(value) => {
                vars.push(code_block! {
                    #name: #value
                });
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(code_block! {
        {
            #(
                #vars,
            )*
        }
    })
}
