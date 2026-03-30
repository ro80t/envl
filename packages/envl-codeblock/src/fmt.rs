use crate::codeblock::CodeBlock;
use syn::File;

#[derive(Debug, Clone)]
pub enum Language {
    Rust,
}

pub fn formatter(lang: Language, code_block: CodeBlock) -> String {
    let basic_formatted_source = code_block.inner.to_string();
    match lang {
        Language::Rust => match syn::parse2::<File>(code_block.inner) {
            Ok(file) => prettyplease::unparse(&file),
            Err(_) => basic_formatted_source,
        },
    }
}
