// Libraries use in this library
mod error;
mod methods;
mod models;
#[cfg(test)]
mod tests;

pub use crate::models::SecureDelete;

#[cfg(feature = "error-stack")]
pub use crate::error::ehanced::{Error, Result};
#[cfg(not(feature = "error-stack"))]
pub use crate::error::standard::{Error, Result};

pub use crate::methods::Method;
