//! Internal execution engine.
//!
//! This module is not part of the public API.
// NOTE (PR-1):
// The engine currently propagates legacy error types to preserve
// exact behavior. Error decoupling is intentionally deferred.

pub(crate) mod events;
mod executor;
mod overwrite;
mod planner;
mod utils;
#[cfg(feature = "verify")]
pub(crate) mod verify;

pub(crate) use executor::run;
#[cfg(feature = "dry-run")]
pub(crate) use executor::dry_run;
#[cfg(feature = "dry-run")]
pub(crate) use utils::emit_safe;
