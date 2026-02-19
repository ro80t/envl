use std::io::Error;

use envl_codeblock::{code_block, codeblock::CodeBlock};
use envl_utils::variable::Value;
use proc_macro2::Literal;

use crate::generator::js::{gen_array::gen_array, gen_struct::gen_struct};

pub(crate) fn gen_value(v: Value) -> Result<CodeBlock, Error> {
    match &v {
        Value::Null => Ok(code_block! {null}),
        Value::String(s) => Ok(code_block! {#s}),
        Value::Char(c) => Ok(code_block! {#c}),
        Value::Float(f) => Ok(CodeBlock::from(Literal::f64_unsuffixed(*f))),
        Value::Int(i) => Ok(CodeBlock::from(Literal::i64_unsuffixed(*i))),
        Value::Uint(u) => Ok(CodeBlock::from(Literal::u64_unsuffixed(*u))),
        Value::Bool(b) => Ok(code_block! {#b}),
        Value::Array(a) => gen_array(a.to_owned()),
        Value::Struct(value) => gen_struct(value.to_owned()),
    }
}
