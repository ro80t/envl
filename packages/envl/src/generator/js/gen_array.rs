use std::io::Error;

use envl_codeblock::code_block;
use envl_config::misc::variable::Value;
use proc_macro2::TokenStream;

use crate::generator::js::value::gen_value;

pub(crate) fn gen_array(v: Vec<Value>) -> Result<TokenStream, Error> {
    let mut vec_values = Vec::new();

    for value in v {
        match gen_value(value.to_owned()) {
            Ok(r) => {
                vec_values.push(r);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(code_block! {
        [
            #(
                #vec_values,
            )*
        ]
    })
}
