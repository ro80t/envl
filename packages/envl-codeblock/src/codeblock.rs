use std::fmt::Display;

use proc_macro2::{Group, Literal, Punct, TokenStream};
use quote::ToTokens;
use syn::Ident;

use crate::fmt::{formatter, Language};

#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub inner: TokenStream,
}

impl CodeBlock {
    pub fn to_string_with_formatter(&self, lang: Language) -> String {
        formatter(lang, self.clone())
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

impl From<Group> for CodeBlock {
    fn from(value: Group) -> Self {
        Self {
            inner: value.to_token_stream(),
        }
    }
}

impl From<Ident> for CodeBlock {
    fn from(value: Ident) -> Self {
        Self {
            inner: value.to_token_stream(),
        }
    }
}

impl From<Punct> for CodeBlock {
    fn from(value: Punct) -> Self {
        Self {
            inner: value.to_token_stream(),
        }
    }
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = self.inner.to_string();
        f.write_str(txt.trim())
    }
}

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.inner.clone());
    }
}
