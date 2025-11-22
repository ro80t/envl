use std::collections::HashMap;

use envl_config::misc::variable::Value;

use crate::{VarData, VariableHashMap};

pub(crate) fn adapt_default_value(hm: VariableHashMap) -> VariableHashMap {
    let mut adapted_hm = HashMap::new();

    for (name, var) in hm {
        match var.value {
            Value::Null => {
                adapted_hm.insert(
                    name,
                    VarData {
                        value: var.default_value.to_owned(),
                        ..var
                    },
                );
            }
            _ => {
                adapted_hm.insert(name, var);
            }
        }
    }

    adapted_hm
}
