// Libraries use in this library
mod error;
#[cfg(test)]
mod tests;
mod methods; 
mod models;

pub use crate::models::SecureDelete;

#[cfg(not(feature="error-stack"))]
pub use crate::error::standard::{Error, Result};

pub use crate::methods::Method;
