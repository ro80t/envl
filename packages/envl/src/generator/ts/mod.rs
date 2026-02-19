use std::{collections::HashMap, io::Error};

use envl_codeblock::{code_block, codeblock::CodeBlock};

use crate::{
    generator::{js::generate_js_file_core, ts::gen_struct::gen_struct, GenerateOptions},
    VariableHashMap,
};

pub mod gen_array;
pub mod gen_struct;
pub mod value;

pub fn generate_ts_file(data: VariableHashMap, options: GenerateOptions) -> Result<String, Error> {
    match generate_js_file_core(&data) {
        Ok(env_value) => {
            let type_file = generate_ts_type_file(&data);
            let base_code = code_block! {
                const envl = #env_value satisfies Envl;
            };

            if options.cjs.is_some_and(|cjs| cjs) {
                Ok(code_block! {
                    #type_file
                    #base_code

                    module.exports = { envl, Envl };
                }
                .to_string())
            } else {
                Ok(code_block! {
                    #type_file
                    #base_code

                    export type { Envl };
                    export { envl };
                }
                .to_string())
            }
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn generate_ts_type_file(data: &VariableHashMap) -> CodeBlock {
    let hm = data
        .iter()
        .map(|(n, v)| (n.clone(), v.v_type.clone()))
        .collect::<HashMap<_, _>>();

    let value = gen_struct(hm);
    code_block! {
        type Envl = #value;
    }
}
