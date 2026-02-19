use std::collections::HashMap;

use envl_codeblock::{code_block, codeblock::CodeBlock};
use envl_utils::variable::Type;
use proc_macro2::TokenStream;

use crate::generator::ts::value::gen_value;

pub(crate) fn gen_struct(hm: HashMap<String, Type>) -> CodeBlock {
    let mut values = Vec::new();

    for (name, value) in hm {
        let n = name.parse::<TokenStream>().unwrap();
        let v = gen_value(value);
        values.push(code_block! {#n: #v});
    }

    code_block! {
        {
            #(
                #values,
            )*
        }
    }
}
