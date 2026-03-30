use std::io::Error;

use envl_codeblock::{code_block, codeblock::CodeBlock};
use envl_utils::variable::{Type, Value};
use proc_macro2::{Literal, TokenStream};

use crate::generator::rust::var::{array::gen_array, gen_struct::gen_struct};

pub fn gen_value(
    name: String,
    t: Type,
    v: Value,
    structs: &mut Vec<TokenStream>,
) -> Result<CodeBlock, Error> {
    let result = match &v {
        Value::Null => Ok(code_block! {None}),
        Value::String(s) => Ok(code_block! {String::from(#s)}),
        Value::Char(c) => Ok(code_block! {#c}),
        Value::Float(f) => Ok(CodeBlock::from(Literal::f64_unsuffixed(*f))),
        Value::Int(i) => Ok(CodeBlock::from(Literal::i64_unsuffixed(*i))),
        Value::Uint(u) => Ok(CodeBlock::from(Literal::u64_unsuffixed(*u))),
        Value::Bool(b) => Ok(code_block! {#b}),
        Value::Array(a) => match &t {
            Type::Array(boxed_type) => {
                match gen_array(
                    format!("Array{}", &name),
                    *boxed_type.to_owned(),
                    a.to_vec(),
                    structs,
                ) {
                    Ok(r) => Ok(r),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::other("Invalid Type")),
        },
        Value::Struct(value) => match &t {
            Type::Struct(struct_type) => {
                match gen_struct(name, struct_type.to_owned(), value.to_owned(), structs) {
                    Ok(r) => Ok(r),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::other("Invalid Type")),
        },
    };

    match result {
        Ok(token) => match t.clone() {
            Type::Option(_) if v != Value::Null => Ok(code_block! {
                Some(#token)
            }),
            _ => Ok(token.to_owned()),
        },
        Err(err) => Err(err),
    }
}
