use std::fmt::Display;

use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::ToTokens;
use syn::Ident;

#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub inner: TokenStream,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BreakLineConfig {
    pub chars: Vec<char>,
    pub brackets: Vec<Delimiter>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpaceConfig {
    pub no_after_space_chars: Vec<char>,
    pub no_before_space_chars: Vec<char>,
    pub no_space_chars: Vec<char>,
    pub brakets: Vec<Delimiter>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CustomConfig {
    pub joint_chars: bool,
    pub space: SpaceConfig,
    pub break_line: BreakLineConfig,
}

impl CustomConfig {
    pub fn plain() -> Self {
        Self {
            joint_chars: true,
            space: SpaceConfig {
                no_after_space_chars: Vec::new(),
                no_before_space_chars: Vec::new(),
                no_space_chars: Vec::new(),
                brakets: Vec::new(),
            },
            break_line: BreakLineConfig {
                chars: Vec::new(),
                brackets: Vec::new(),
            },
        }
    }
}

impl Default for CustomConfig {
    fn default() -> Self {
        Self {
            joint_chars: true,
            space: SpaceConfig {
                no_after_space_chars: vec![':'],
                no_before_space_chars: Vec::new(),
                no_space_chars: vec![',', ';', '.'],
                brakets: vec![Delimiter::Parenthesis],
            },
            break_line: BreakLineConfig {
                chars: vec![';'],
                brackets: vec![Delimiter::Brace],
            },
        }
    }
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

    fn token_to_char(&self, index: usize, tokens: Vec<TokenTree>) -> Option<char> {
        if let Some(token) = tokens.get(index) {
            match token {
                TokenTree::Group(_group) => None,
                TokenTree::Ident(ident) => {
                    let txt = ident.to_string();
                    match txt.chars().collect::<Vec<_>>().first() {
                        Some(ch) if txt.len() == 1 => Some(*ch),
                        None => None,
                        _ => None,
                    }
                }
                TokenTree::Literal(literal) => {
                    let txt = literal.to_string();
                    match txt.chars().collect::<Vec<_>>().first() {
                        Some(ch) if txt.len() == 1 => Some(*ch),
                        None => None,
                        _ => None,
                    }
                }
                TokenTree::Punct(punct) => Some(punct.as_char()),
            }
        } else {
            None
        }
    }

    fn get_space_after_token(
        &self,
        config: CustomConfig,
        index: usize,
        tokens: Vec<TokenTree>,
    ) -> String {
        let next_index = index + 1;

        let curr_char = self.token_to_char(index, tokens.clone());
        let next_char = self.token_to_char(next_index, tokens.clone());

        if let Some(ch) = curr_char {
            if config.break_line.chars.contains(&ch) {
                return String::from("\n");
            }
            if config.space.no_after_space_chars.contains(&ch)
                || config.space.no_space_chars.contains(&ch)
            {
                return String::new();
            }
        }

        if let Some(ch) = next_char {
            if config.space.no_before_space_chars.contains(&ch)
                || config.space.no_space_chars.contains(&ch)
            {
                return String::new();
            }
        }

        if !config.space.brakets.is_empty() {
            if let Some(TokenTree::Group(group)) = tokens.get(next_index) {
                if config.space.brakets.contains(&group.delimiter()) {
                    return String::new();
                }
            }
        }
        " ".to_string()
    }

    fn token_stream_to_string(
        &self,
        config: CustomConfig,
        token_stream: TokenStream,
        indent: &mut u64,
    ) -> String {
        let tokens = token_stream.into_iter().collect::<Vec<_>>();
        let mut txt = String::new();
        let mut is_first = true;

        for (i, token) in tokens.iter().enumerate() {
            let space = self.get_space_after_token(config.clone(), i, tokens.clone());

            match token {
                TokenTree::Group(group) => {
                    let delimiter = group.delimiter();
                    if config.break_line.brackets.contains(&delimiter) {
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
                            space.as_str(),
                        ]);
                    }
                }
                TokenTree::Ident(ident) => {
                    txt.extend([
                        self.reflect_indent(ident.to_string(), indent, is_first)
                            .as_str(),
                        space.as_str(),
                    ]);
                }
                TokenTree::Literal(literal) => {
                    txt.extend([
                        self.reflect_indent(literal.to_string(), indent, is_first)
                            .as_str(),
                        space.as_str(),
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
                        if config.joint_chars && is_joint {
                            ""
                        } else {
                            space.as_str()
                        },
                    ]);
                }
            }
            is_first = space == "\n";
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
        let txt = self.to_string_with_custom_config(CustomConfig::plain());
        f.write_str(txt.trim())
    }
}

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.inner.clone());
    }
}
