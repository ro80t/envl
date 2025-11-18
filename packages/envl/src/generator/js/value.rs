use std::io::Error;

use envl_codeblock::code_block;
use envl_config::misc::variable::Value;
use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;

use crate::generator::js::{gen_array::gen_array, gen_struct::gen_struct};

pub(crate) fn gen_value(v: Value) -> Result<TokenStream, Error> {
    match &v {
        Value::Null => Ok(code_block! {null}),
        Value::String(s) => Ok(code_block! {#s}),
        Value::Char(c) => Ok(code_block! {#c}),
        Value::Float(f) => Ok(Literal::f64_unsuffixed(*f).into_token_stream()),
        Value::Int(i) => Ok(Literal::i64_unsuffixed(*i).to_token_stream()),
        Value::Uint(u) => Ok(Literal::u64_unsuffixed(*u).to_token_stream()),
        Value::Bool(b) => Ok(code_block! {#b}),
        Value::Array(a) => gen_array(a.to_owned()),
        Value::Struct(value) => gen_struct(value.to_owned()),
    }
}
