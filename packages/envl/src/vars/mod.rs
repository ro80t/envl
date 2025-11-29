use std::collections::HashMap;

use envl_config::misc::config::Config;
use envl_utils::{
    error::{EnvlError, ErrorContext},
    types::Position,
    variable::{Type, Value as VarVariable},
};

use crate::{
    misc::vars::Value,
    vars::{type_check::type_check, var::gen_variable_data},
    VarData, VariableHashMap,
};

pub mod type_check;
pub mod var;

pub(crate) fn gen_vars(
    var_file_path: String,
    vars: HashMap<String, Value>,
    config: Config,
) -> Result<VariableHashMap, EnvlError> {
    let mut no_used_vars = Vec::new();
    let mut hm = HashMap::new();

    for (name, var) in config.vars.clone() {
        if let Some(value) = vars.get(&name) {
            match gen_variable_data(&name, value.clone(), var.v_type.clone()) {
                Ok(var_data) => {
                    hm.insert(
                        name,
                        VarData {
                            value: var_data,
                            position: value.position.clone(),
                            v_type: var.v_type,
                            default_value: var.default_value,
                            actions_value: var.actions_value,
                        },
                    );
                }
                Err(err) => {
                    return Err(err);
                }
            }
        } else {
            match var.default_value {
                VarVariable::Null => match var.v_type {
                    Type::Option(_) => {}
                    _ => {
                        return Err(EnvlError {
                            message: ErrorContext::Required(format!("{}", name)),
                            position: Position::default(),
                        });
                    }
                },
                _ => {}
            }

            if let Err(err) = type_check(
                &name,
                var.default_value.clone(),
                var.v_type.clone(),
                &var.position,
            ) {
                return Err(err);
            }

            hm.insert(
                name,
                VarData {
                    value: var.default_value.clone(),
                    v_type: var.v_type,
                    default_value: var.default_value,
                    actions_value: var.actions_value,
                    position: var.position,
                },
            );
        }
    }

    for (name, _) in vars {
        if config.vars.get(&name).is_none() {
            no_used_vars.push(name);
        }
    }

    if no_used_vars.len() != 0 {
        return Err(EnvlError {
            message: ErrorContext::UnnecessaryVariable(no_used_vars.join(" and ")),
            position: Position {
                file_path: var_file_path,
                col: 0,
                row: 0,
            },
        });
    }

    Ok(hm)
}
