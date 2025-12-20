use envl_config::{generate_ast as gen_config_ast, misc::config::Config};
use envl_utils::types::FilePosition;
use envl_utils::variable::{Type, Value};
use envl_utils::{error::EnvlError, types::Position};
use envl_variables::{generate_ast as gen_vars_ast, misc::variable::Variable};
use std::{collections::HashMap, env::current_dir, path::PathBuf};

use crate::vars::gen_vars;
use crate::{
    generator::{generate_file, GenerateOptions},
    misc::{
        error::convert_io_error,
        filesystem::{read_file, write_file},
        vars::vars_to_hashmap,
    },
};

pub mod generator;
pub mod misc;
pub mod vars;

#[derive(Debug, Clone)]
pub struct VarData {
    pub value: Value,
    pub v_type: Type,
    pub default_value: Value,
    pub actions_value: Value,
    pub position: Position,
}

pub type VariableHashMap = HashMap<String, VarData>;

pub fn load_envl(output: String) -> Result<(), Box<EnvlError>> {
    load_envl_with_options(output, GenerateOptions::default())
}

pub fn load_envl_with_options(
    output: String,
    options: GenerateOptions,
) -> Result<(), Box<EnvlError>> {
    match current_dir() {
        Ok(current_dir_path) => {
            let config_file_path = current_dir_path.join(".envlconf").display().to_string();
            match read_file(config_file_path.to_owned()) {
                Ok(code) => {
                    match load_envl_core(
                        current_dir_path.to_owned(),
                        config_file_path.to_owned(),
                        code,
                    ) {
                        Ok(hm) => match generate_file(hm, output.to_owned(), options) {
                            Ok(result) => {
                                if let Err(err) = write_file(output, result) {
                                    Err(Box::from(convert_io_error(
                                        err,
                                        Position {
                                            file_path: file!().to_string(),
                                            start: FilePosition {
                                                row: line!() as usize,
                                                col: column!() as usize,
                                            },
                                            end: FilePosition {
                                                row: line!() as usize,
                                                col: column!() as usize,
                                            },
                                        },
                                    )))
                                } else {
                                    Ok(())
                                }
                            }
                            Err(err) => Err(Box::from(convert_io_error(
                                err,
                                Position {
                                    file_path: file!().to_string(),
                                    start: FilePosition {
                                        row: line!() as usize,
                                        col: column!() as usize,
                                    },
                                    end: FilePosition {
                                        row: line!() as usize,
                                        col: column!() as usize,
                                    },
                                },
                            ))),
                        },
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(Box::from(convert_io_error(
            err,
            Position {
                file_path: file!().to_string(),
                start: FilePosition {
                    row: line!() as usize,
                    col: column!() as usize,
                },
                end: FilePosition {
                    row: line!() as usize,
                    col: column!() as usize,
                },
            },
        ))),
    }
}

pub fn load_envl_core(
    current_dir: PathBuf,
    config_file_path: String,
    code: String,
) -> Result<VariableHashMap, Box<EnvlError>> {
    match load_files(current_dir, config_file_path, code) {
        Ok((vars, config, var_file_path)) => {
            let vars_hm = vars_to_hashmap(vars);

            match gen_vars(var_file_path, vars_hm, config) {
                Ok(hm) => Ok(hm),
                Err(err) => Err(Box::from(err)),
            }
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn load_files(
    current_dir: PathBuf,
    config_file_path: String,
    code: String,
) -> Result<(Vec<Variable>, Config, String), Box<EnvlError>> {
    match gen_config_ast(config_file_path.clone(), code.clone()) {
        Ok(config) => {
            let file_path = if let Some(ref file_path) = config.settings.envl_file_path {
                file_path.value.clone()
            } else {
                current_dir.join(".envl").display().to_string()
            };
            match read_file(file_path.to_owned()) {
                Ok(code) => match gen_vars_ast(file_path.clone(), code) {
                    Ok(vars) => Ok((vars, config, file_path)),
                    Err(err) => Err(Box::from(err)),
                },
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(Box::from(err)),
    }
}
