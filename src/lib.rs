//! # Nozomi
//!
//! Nozomi is a Rust library for **secure file deletion**. It implements several
//! industry-standard overwrite methods that prevent data recovery by overwriting
//! file contents with specific byte patterns before removing the file from the
//! filesystem.
//!
//! ## Supported sanitisation standards
//!
//! | Variant | Standard | Passes |
//! |---------|----------|--------|
//! | [`Method::PseudoRandom`] | Random data | 1 |
//! | [`Method::HmgiS5`] | HMGI S5 | 2 |
//! | [`Method::Afssi5020`] | AFSSI 5020 | 3 |
//! | [`Method::Dod522022ME`] | DoD 5220.22-M (ME) | 3 |
//! | [`Method::RcmpTssitOpsII`] | RCMP TSSIT OPS-II | 7 |
//! | [`Method::Dod522022MECE`] | DoD 5220.22-M (ECE) | 7 |
//! | [`Method::Gutmann`] | Gutmann | 35 |
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use nozomi::{DeleteRequest, DeleteMethod, Method};
//!
//! let report = DeleteRequest::builder()
//!     .path("/path/to/sensitive/file.txt")
//!     .method(DeleteMethod::BuiltIn(Method::Gutmann))
//!     .build()?
//!     .run()?;
//!
//! println!("Deleted {:?} using {}", report.path, report.method);
//! # Ok::<(), nozomi::Error>(())
//! ```
//!
//! ## Optional features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `dry-run` | Simulate deletions without writing to disk |
//! | `verify` | Read back the last overwrite pass to confirm correctness |
//! | `analyze` | Inspect the pass schedule of a [`Method`] before running it |
//! | `error-stack` | Use [`error_stack`](https://docs.rs/error-stack) for richer error context (deprecated, will be removed in `4.0.0`) |
//! | `log` | Emit trace-level log entries via the `log` facade |

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
