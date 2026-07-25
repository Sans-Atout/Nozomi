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
//! | `log` | Emit trace-level log entries via the `log` facade |

// -- Libraries use in this library
#[cfg(feature = "analyze")]
mod analyze;
mod error;
mod methods;
#[cfg(test)]
mod tests;
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

// -- Export Error and Result type
pub use crate::error::{Error, ExecutionError, IoError, PlanningError, Result};
