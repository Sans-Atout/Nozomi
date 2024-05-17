#[cfg(feature = "error-stack")]
use error_stack::{self, Context};
use std::{fmt, path::Path};

use crate::models::SecureDelete;

/// 
#[cfg(feature = "error-stack")]
#[derive(Debug, Clone, Copy)]
pub struct InputError;

#[cfg(feature = "error-stack")]
impl fmt::Display for InputError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Input Error : invalid given input")
    }
}

#[cfg(feature = "error-stack")]
impl Context for InputError {}

#[cfg(feature = "error-stack")]
#[derive(Debug, Clone, Copy)]
pub struct ProcessError;

#[cfg(feature = "error-stack")]
impl fmt::Display for ProcessError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Process Error : overwrite method fail")
    }
}

#[cfg(feature = "error-stack")]
impl Context for ProcessError {}
/// End of Error-Stack Zone

#[cfg(not(feature = "error-stack"))]
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(not(feature = "error-stack"))]
#[derive(Debug)]
pub enum Error {
    InputError(String),
    FileNotFound(String),
    OverwriteError(SecureDelete),
    DeleteError(SecureDelete),
    NoFileName(SecureDelete)

}