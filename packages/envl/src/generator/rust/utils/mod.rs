use envl_codeblock::{code_block, codeblock::CodeBlock};

pub fn struct_derive() -> CodeBlock {
    code_block! {
        #[derive(Debug, Clone, PartialEq)]
    }
}
