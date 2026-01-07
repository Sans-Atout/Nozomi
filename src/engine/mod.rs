//! Internal execution engine.
//!
//! This module is not part of the public API.

mod executor;
mod planner;
mod overwrite;

pub(crate) use executor::run;
