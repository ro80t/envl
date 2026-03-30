use envl_codeblock::{code_block, codeblock::CodeBlock};
use envl_utils::variable::Type;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::generator::rust::utils::struct_derive;

pub fn parse_v_type(v_name: String, v_type: Type, structs: &mut Vec<TokenStream>) -> CodeBlock {
    match v_type {
        Type::Array(boxed_element_type) => {
            let value = parse_v_type(format!("Array{}", v_name), *boxed_element_type, structs);
            code_block! {
                Vec<#value>
            }
        }
        Type::Bool => code_block! {bool},
        Type::Char => code_block! {char},
        Type::Float => code_block! {f64},
        Type::Int => code_block! {i64},
        Type::Null => code_block! {None},
        Type::String => code_block! {String},
        Type::Option(t) => {
            let value = parse_v_type(v_name, *t, structs);
            code_block! {
                Option<#value>
            }
        }
        Type::Struct(elements) => {
            let s_derive = struct_derive();
            let struct_name = format!("Struct{}", v_name).parse::<TokenStream>().unwrap();
            let struct_value = elements
                .iter()
                .map(|(n, v)| {
                    let name = match v {
                        Type::Struct(_) => {
                            format!("{}{}", struct_name, n)
                        }
                        _ => n.to_string(),
                    };
                    let token_stream_name = n.parse::<TokenStream>().unwrap();
                    let v_type = parse_v_type(name.to_owned(), v.to_owned(), structs);
                    code_block! {#token_stream_name: #v_type}
                })
                .collect::<Vec<_>>();

            structs.push(
                code_block! {
                    #s_derive
                    #[rustfmt::skip]
                    pub struct #struct_name {
                        #(
                            pub #struct_value,
                        )*
                    }
                }
                .to_token_stream(),
            );

            code_block! {
                #struct_name
            }
        }
        Type::Uint => code_block! {u64},
    }
}
