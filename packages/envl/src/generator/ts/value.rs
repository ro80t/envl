use envl_codeblock::code_block;
use envl_utils::variable::Type;
use proc_macro2::TokenStream;

use crate::generator::ts::{gen_array::gen_array, gen_struct::gen_struct};

pub(crate) fn gen_value(t: Type) -> TokenStream {
    match &t {
        Type::Array(boxed_element_type) => gen_array(*boxed_element_type.clone()),
        Type::Bool => code_block! {boolean},
        Type::Char => code_block! {string},
        Type::Float => code_block! {number},
        Type::Int => code_block! {number},
        Type::Null => code_block! {null},
        Type::String => code_block! {string},
        Type::Option(t) => {
            let gened_type = gen_value(*t.clone());
            code_block! {
                #gened_type | undefined
            }
        }
        Type::Struct(elements) => gen_struct(elements.clone()),
        Type::Uint => code_block! {number},
    }
}
