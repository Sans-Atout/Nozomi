// -- Libraries use in this library
mod error;
mod methods;
mod models;
#[cfg(test)]
mod tests;

// -- Export object
pub use crate::methods::Method;
pub use crate::models::SecureDelete;

// -- Export Error and Result type
#[cfg(feature = "error-stack")]
pub use crate::error::enhanced::{Error, Result};
#[cfg(not(feature = "error-stack"))]
pub use crate::error::standard::{Error, Result};
