use std::fmt::Display;

use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;

#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub inner: TokenStream,
}

impl CodeBlock {
    pub fn to_plain_string(&self) -> String {
        self.inner.to_string()
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
        f.write_str(&self.to_plain_string())
    }
}

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.inner.clone());
    }
}
