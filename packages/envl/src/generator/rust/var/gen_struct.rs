use std::{collections::HashMap, io::Error};

use envl_codeblock::code_block;
use envl_codeblock::codeblock::CodeBlock;
use envl_utils::case::{CamelCase, Case, SnakeCase};
use envl_utils::variable::{Type, Value};
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::generator::rust::var::gen_value;

pub fn gen_struct(
    name: String,
    t: HashMap<String, Type>,
    v: HashMap<String, Value>,
    structs: &mut Vec<TokenStream>,
) -> Result<CodeBlock, Error> {
    let struct_type = format!("Struct{}", name).parse::<TokenStream>().unwrap();
    let struct_name = SnakeCase::gen(CamelCase::parse(format!("struct{}", name).as_str()))
        .parse::<TokenStream>()
        .unwrap();
    let mut struct_values = Vec::new();

    for (n, element_type) in t {
        if let Some(value) = v.get(&n) {
            let element_name = match value {
                Value::Struct(_) => {
                    format!("{}{}", struct_type, n.to_owned())
                }
                _ => n.to_owned(),
            };
            match gen_value(
                element_name.to_owned(),
                element_type.to_owned(),
                value.to_owned(),
                structs,
            ) {
                Ok(r) => {
                    struct_values.push((n.to_owned(), r));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        } else {
            return Err(Error::other(format!("{} is required", &n)));
        }
    }

    let elements = struct_values
        .iter()
        .map(|(n, v)| {
            let name = n.parse::<TokenStream>().unwrap();
            code_block! {#name: #v}
        })
        .collect::<Vec<_>>();

    structs.push(
        code_block! {
            let #struct_name = #struct_type {
                #(
                    #elements,
                )*
            };
        }
        .to_token_stream(),
    );

    Ok(code_block! {
        #struct_name
    })
}
