use std::fmt::Display;

use proc_macro2::{Literal, Spacing, TokenStream, TokenTree};
use quote::ToTokens;

#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub inner: TokenStream,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomConfig {}

impl CodeBlock {
    pub fn to_string_with_custom_config(&self, _config: CustomConfig) -> String {
        let mut txt = String::new();

        for token in self.inner.to_token_stream() {
            match token {
                TokenTree::Group(group) => {
                    txt.extend([group.to_string().as_str(), " "]);
                }
                TokenTree::Ident(ident) => {
                    txt.extend([ident.to_string().as_str(), " "]);
                }
                TokenTree::Literal(literal) => {
                    txt.extend([literal.to_string().as_str(), " "]);
                }
                TokenTree::Punct(punct) => {
                    let is_joint = match punct.spacing() {
                        Spacing::Alone => false,
                        Spacing::Joint => true,
                    };
                    txt.extend([punct.to_string().as_str(), if is_joint { "" } else { " " }]);
                }
            }
        }

        txt
    }
}

impl From<TokenStream> for CodeBlock {
    fn from(value: TokenStream) -> Self {
        Self { inner: value }
    }
}

impl From<Literal> for CodeBlock {
    fn from(value: Literal) -> Self {
        Self {
            inner: value.to_token_stream(),
        }
    }
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = self.to_string_with_custom_config(CustomConfig::default());
        f.write_str(txt.trim())
    }
}

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.inner.clone());
    }
}
