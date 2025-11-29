use envl_utils::{
    error::{EnvlError, ErrorContext},
    types::Position,
    variable::{Type, Value},
};

pub(crate) fn type_check(
    var_name: &String,
    var: Value,
    v_type: Type,
    position: &Position,
) -> Result<(), EnvlError> {
    let invalid_type_error = EnvlError {
        message: ErrorContext::InvalidType,
        position: position.clone(),
    };

    match &v_type {
        Type::Array(t) => match var {
            Value::Array(v) => {
                for e in v {
                    type_check(var_name, e, *t.clone(), position)?
                }
                Ok(())
            }
            _ => Err(invalid_type_error),
        },
        Type::Bool => match var {
            Value::Bool(_) => Ok(()),
            _ => Err(invalid_type_error),
        },
        Type::Char => match var {
            Value::Char(_) => Ok(()),
            _ => Err(invalid_type_error),
        },
        Type::Float => match var {
            Value::Float(_) => Ok(()),
            _ => Err(invalid_type_error),
        },
        Type::Int => match var {
            Value::Int(_) => Ok(()),
            _ => Err(invalid_type_error),
        },
        Type::Null => match var {
            Value::Null => Ok(()),
            _ => Err(invalid_type_error),
        },
        Type::Option(t) => match var {
            Value::Null => Ok(()),
            other => type_check(var_name, other, *t.clone(), position),
        },
        Type::String => match var {
            Value::String(_) => Ok(()),
            _ => Err(invalid_type_error),
        },
        Type::Struct(t) => match var {
            Value::Struct(v) => {
                for (name, v_type) in t {
                    if let Some(value) = v.get(name) {
                        match type_check(name, value.clone(), v_type.clone(), position) {
                            Ok(_) => {}
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else {
                        return Err(EnvlError {
                            message: ErrorContext::Required(format!(
                                "{} in struct {}",
                                name, var_name
                            )),
                            position: position.clone(),
                        });
                    }
                }
                Ok(())
            }
            _ => Err(invalid_type_error),
        },
        Type::Uint => match var {
            Value::Uint(_) => Ok(()),
            _ => Err(invalid_type_error),
        },
    }
}
