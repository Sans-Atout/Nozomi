// -- Libraries use in this library
#[cfg(feature = "analyze")]
mod analyze;
#[allow(deprecated)]
mod error;
mod methods;
#[allow(deprecated)]
mod models;
#[cfg(test)]
#[allow(deprecated)]
mod tests;
#[cfg(feature = "analyze")]
pub use analyze::{AnalysisReport, PassInfo, PassKind};

#[allow(deprecated)]
pub mod api;
#[allow(deprecated)]
mod engine;

pub use api::delete::DeleteMethod;
pub use api::delete::DeleteReport;
pub use api::delete::DeleteRequest;
pub use api::delete::DeleteRequestBuilder;

pub use engine::events::{DeleteEvent, EventSink};

// -- Export object
pub use crate::methods::Method;
#[allow(deprecated)]
pub use crate::models::SecureDelete;

// -- Export Error and Result type
#[cfg(feature = "error-stack")]
#[allow(deprecated)]
pub use crate::error::enhanced::{Error, Result};
#[cfg(not(feature = "error-stack"))]
pub use crate::error::standard::{Error, Result};
