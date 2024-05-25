// Libraries use in this library
mod error;
#[cfg(test)]
pub mod tests;
pub mod methods; 
pub mod models;

#[cfg(not(feature = "error-stack"))]
use crate::error::{Error, Result};
use crate::methods::Method;