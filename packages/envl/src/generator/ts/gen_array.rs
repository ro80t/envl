use envl_codeblock::code_block;
use envl_utils::variable::Type;
use proc_macro2::TokenStream;

use crate::generator::ts::value::gen_value;

pub(crate) fn gen_array(t: Type) -> TokenStream {
    let element_type = gen_value(t);
    code_block! {
        #element_type[]
    }
}
