use std::{
    fs::{File, OpenOptions},
    io::{Error, Read, Write},
    path::Path,
};

use envl_utils::{
    error::EnvlError,
    types::{FilePosition, Position},
};

use crate::misc::error::convert_io_error;

pub fn read_file(file_path: String) -> Result<String, Box<EnvlError>> {
    match File::open(&file_path) {
        Ok(mut f) => {
            let mut buf = String::new();
            let _ = f.read_to_string(&mut buf);
            Ok(buf)
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

pub fn write_file(file_path: String, txt: String) -> Result<usize, Error> {
    let f = if Path::new(&file_path).is_file() {
        OpenOptions::new().write(true).open(&file_path)
    } else {
        File::create(file_path)
    };
    match f {
        Ok(mut f) => {
            let _ = f.write("".as_bytes());
            f.write(format!("{}\n", txt).as_bytes())
        }
        Err(err) => Err(err),
    }
}
