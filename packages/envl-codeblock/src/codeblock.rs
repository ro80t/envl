use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;

pub struct CodeBlock {
    pub inner: TokenStream,
}

impl ToString for CodeBlock {
    fn to_string(&self) -> String {
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

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.inner.clone());
    }
}
