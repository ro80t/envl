use std::collections::HashMap;

use envl_utils::{
    error::{EnvlError, ErrorContext},
    variable::{Type, Value as VariableHmValue},
};
use envl_vars::misc::variable::VariableValue;

use crate::misc::vars::Value;

pub(crate) fn gen_variable_data(
    var_name: &String,
    var: Value,
    v_type: Type,
) -> Result<VariableHmValue, EnvlError> {
    let position = var.clone().position;

    match v_type {
        Type::Option(t) => gen_variable_data(var_name, var, *t),
        _ => match &var.value {
            VariableValue::Array(v) => match v_type {
                Type::Array(boxed_e_type) => {
                    let e_type = *boxed_e_type;
                    let mut arr = Vec::new();

                    for value in v {
                        match gen_variable_data(
                            var_name,
                            Value {
                                value: value.clone(),
                                position: position.clone(),
                            },
                            e_type.clone(),
                        ) {
                            Ok(e) => {
                                arr.push(e);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }

                    Ok(VariableHmValue::Array(arr))
                }
                _ => Err(EnvlError {
                    message: ErrorContext::InvalidType,
                    position,
                }),
            },
            VariableValue::Bool(v) => match v_type {
                Type::Bool => Ok(VariableHmValue::Bool(*v)),
                _ => Err(EnvlError {
                    message: ErrorContext::InvalidType,
                    position,
                }),
            },
            VariableValue::Char(v) => match v_type {
                Type::Char => Ok(VariableHmValue::Char(*v)),
                _ => Err(EnvlError {
                    message: ErrorContext::InvalidType,
                    position,
                }),
            },
            VariableValue::Number(v) => match v_type {
                Type::Float => {
                    if let Ok(num) = v.parse::<f64>() {
                        Ok(VariableHmValue::Float(num))
                    } else {
                        Err(EnvlError {
                            message: ErrorContext::InvalidType,
                            position,
                        })
                    }
                }
                Type::Int => {
                    if let Ok(num) = v.parse::<i64>() {
                        Ok(VariableHmValue::Int(num))
                    } else {
                        Err(EnvlError {
                            message: ErrorContext::InvalidType,
                            position,
                        })
                    }
                }
                Type::Uint => {
                    if let Ok(num) = v.parse::<u64>() {
                        Ok(VariableHmValue::Uint(num))
                    } else {
                        Err(EnvlError {
                            message: ErrorContext::InvalidType,
                            position,
                        })
                    }
                }
                _ => Err(EnvlError {
                    message: ErrorContext::InvalidType,
                    position,
                }),
            },
            VariableValue::String(v) => match v_type {
                Type::String => Ok(VariableHmValue::String(v.clone())),
                _ => Err(EnvlError {
                    message: ErrorContext::InvalidType,
                    position,
                }),
            },
            VariableValue::Struct(v) => match v_type {
                Type::Struct(types) => {
                    let mut no_used_vars = Vec::new();
                    let mut hm = HashMap::new();

                    for (n, t) in &types {
                        if let Some(value) = v.get(n) {
                            match gen_variable_data(
                                var_name,
                                Value {
                                    value: value.clone(),
                                    position: position.clone(),
                                },
                                t.clone(),
                            ) {
                                Ok(data) => {
                                    hm.insert(n.clone(), data);
                                }
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        } else {
                            return Err(EnvlError {
                                message: ErrorContext::Required(format!(
                                    "{} in struct {}",
                                    n, var_name
                                )),
                                position,
                            });
                        }
                    }

                    for name in v.keys() {
                        if !types.contains_key(name) {
                            no_used_vars.push(name.to_owned());
                        }
                    }

                    if !no_used_vars.is_empty() {
                        return Err(EnvlError {
                            message: ErrorContext::UnnecessaryVariable(no_used_vars.join(" and ")),
                            position,
                        });
                    }

                    Ok(VariableHmValue::Struct(hm))
                }
                _ => Err(EnvlError {
                    message: ErrorContext::InvalidType,
                    position,
                }),
            },
        },
    }
}
