// -- Libraries use in this library
mod error;
mod methods;
mod models;
#[cfg(test)]
mod tests;
#[cfg(feature = "analyze")]
mod analyze;
#[cfg(feature = "analyze")]
pub use analyze::{AnalysisReport, PassInfo, PassKind};

pub mod api;
mod engine;

pub use api::delete::DeleteMethod;
pub use api::delete::DeleteReport;
pub use api::delete::DeleteRequest;
pub use api::delete::DeleteRequestBuilder;

pub use engine::events::{DeleteEvent, EventSink};

// -- Export object
pub use crate::methods::Method;
pub use crate::models::SecureDelete;

// -- Export Error and Result type
#[cfg(feature = "error-stack")]
pub use crate::error::enhanced::{Error, Result};
#[cfg(not(feature = "error-stack"))]
pub use crate::error::standard::{Error, Result};