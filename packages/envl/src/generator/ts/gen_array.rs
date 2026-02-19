use envl_codeblock::{code_block, codeblock::CodeBlock};
use envl_utils::variable::Type;

use crate::generator::ts::value::gen_value;

pub(crate) fn gen_array(t: Type) -> CodeBlock {
    let element_type = gen_value(t);
    code_block! {
        #element_type[]
    }
}
