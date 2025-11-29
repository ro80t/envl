use std::io::Error;

use envl_utils::{
    error::{EnvlError, ErrorContext},
    types::Position,
};

pub(crate) fn convert_io_error(err: Error, position: Position) -> EnvlError {
    let message = match err.get_ref() {
        Some(msg) => msg.to_string(),
        None => "unknown error".to_string(),
    };

    EnvlError {
        message: ErrorContext::TranspileError(message),
        position,
    }
}
