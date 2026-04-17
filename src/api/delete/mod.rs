//! High-level API for scheduling and executing secure file deletion.
//!
//! This module exposes the primary entry points for consumers of the library.
//! The recommended workflow is to construct a [`DeleteRequest`] via
//! [`DeleteRequestBuilder`], then call [`DeleteRequest::run`] (or
//! [`DeleteRequest::run_with`] to receive structured progress events).
//!
//! # Quick start
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
//! println!("Securely deleted {:?} using {}", report.path, report.method);
//! # Ok::<(), nozomi::Error>(())
//! ```
//!
//! ## Submodules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`builder`] | Fluent builder for [`DeleteRequest`] |
//! | [`report`]  | Result type returned after a successful deletion |
//! | [`request`] | Core request type and deletion method selector |
//! | [`legacy`]  | Compatibility bridges for the deprecated [`SecureDelete`](crate::SecureDelete) API |

pub mod builder;
pub mod legacy;
pub mod report;
pub mod request;

pub use builder::DeleteRequestBuilder;
pub use report::DeleteReport;
pub use request::DeleteMethod;
pub use request::DeleteRequest;
