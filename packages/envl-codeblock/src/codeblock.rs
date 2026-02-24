use std::fmt::Display;

use proc_macro2::{Delimiter, Literal, Spacing, TokenStream, TokenTree};
use quote::ToTokens;

#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub inner: TokenStream,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomConfig {
    pub line_break_brackets: Vec<Delimiter>,
}

impl CodeBlock {
    fn reflect_indent(&self, tokens: String, indent: &mut u64, is_first: bool) -> String {
        let mut txt = String::new();

        if is_first {
            for _ in 0..*indent {
                txt.extend(["\t"]);
            }
        }

        txt.extend([tokens]);

        txt
    }

    fn delimiter_to_string(&self, delimiter: Delimiter) -> (String, String) {
        match delimiter {
            Delimiter::Brace => ("{".to_string(), "}".to_string()),
            Delimiter::Bracket => ("[".to_string(), "]".to_string()),
            Delimiter::Parenthesis => ("(".to_string(), ")".to_string()),
            Delimiter::None => ("".to_string(), "".to_string()),
        }
    }

    fn token_stream_to_string(
        &self,
        config: CustomConfig,
        token_stream: TokenStream,
        indent: &mut u64,
    ) -> String {
        let mut txt = String::new();
        let mut is_first = true;

        for token in token_stream {
            match token {
                TokenTree::Group(group) => {
                    let delimiter = group.delimiter();
                    if config.line_break_brackets.contains(&delimiter) {
                        let delimiter_txt = self.delimiter_to_string(delimiter);
                        txt.extend([
                            self.reflect_indent(delimiter_txt.0, indent, is_first)
                                .as_str(),
                            "\n",
                        ]);
                        *indent += 1;
                        txt.extend([
                            self.token_stream_to_string(config.clone(), group.stream(), indent)
                                .trim_end()
                                .to_string()
                                .as_str(),
                            "\n",
                        ]);
                        *indent -= 1;
                        txt.extend([
                            self.reflect_indent(delimiter_txt.1, indent, is_first)
                                .as_str(),
                            "\n",
                        ]);
                    } else {
                        txt.extend([
                            self.reflect_indent(group.to_string(), indent, is_first)
                                .as_str(),
                            " ",
                        ]);
                    }
                }
                TokenTree::Ident(ident) => {
                    txt.extend([
                        self.reflect_indent(ident.to_string(), indent, is_first)
                            .as_str(),
                        " ",
                    ]);
                }
                TokenTree::Literal(literal) => {
                    txt.extend([
                        self.reflect_indent(literal.to_string(), indent, is_first)
                            .as_str(),
                        " ",
                    ]);
                }
                TokenTree::Punct(punct) => {
                    let is_joint = match punct.spacing() {
                        Spacing::Alone => false,
                        Spacing::Joint => true,
                    };
                    txt.extend([
                        self.reflect_indent(punct.to_string(), indent, is_first)
                            .as_str(),
                        if is_joint { "" } else { " " },
                    ]);
                }
            }
            is_first = false;
        }

        txt
    }

    pub fn to_string_with_custom_config(&self, config: CustomConfig) -> String {
        self.token_stream_to_string(config, self.inner.clone().into_token_stream(), &mut 0)
            .trim()
            .to_string()
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
