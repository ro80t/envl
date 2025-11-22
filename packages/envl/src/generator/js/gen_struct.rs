use std::{collections::HashMap, io::Error};

use envl_codeblock::code_block;
use envl_utils::variable::Value;
use proc_macro2::TokenStream;

use crate::generator::js::value::gen_value;

pub(crate) fn gen_struct(v: HashMap<String, Value>) -> Result<TokenStream, Error> {
    let mut struct_value = Vec::new();

    for (name, value) in v {
        let n = name.parse::<TokenStream>().unwrap();

        match gen_value(value) {
            Ok(element_value) => {
                struct_value.push(code_block! {#n: #element_value});
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(code_block! {
        {
            #(
                #struct_value,
            )*
        }
    })
}
