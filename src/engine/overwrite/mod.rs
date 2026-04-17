//! Method dispatcher for the overwrite engine.
//!
//! Each submodule implements a specific sanitisation standard. This module
//! provides a single [`overwrite_file`] entry point that routes to the correct
//! implementation based on the chosen [`Method`].

// -- Region : Module export
mod afssi_5020;
mod common;
mod dod_522022_me;
mod dod_522022_mece;
mod gutmann;
mod hmgi_s5;
mod pseudo_random;
mod rcmp_tssit_ops_ii;

use crate::{EventSink, Method};
use std::path::Path;

#[cfg(not(feature = "error-stack"))]
use crate::Result;

#[cfg(feature = "error-stack")]
use crate::Result;

/// Overwrites the file at `path` using the algorithm specified by `method`,
/// forwarding progress events to `sink`.
///
/// This is the single dispatch point used by the executor. Each method
/// submodule owns the concrete pass implementation.
///
/// # Errors
///
/// Returns an error if any overwrite pass fails (I/O error or permission
/// denied).
#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file<S: EventSink>(
    method: &Method,
    path: &Path,
    sink: &mut S,
) -> Result<()> {
    match method {
        Method::Dod522022MECE => dod_522022_mece::overwrite_file(path, sink)?,
        Method::Dod522022ME => dod_522022_me::overwrite_file(path, sink)?,
        Method::Afssi5020 => afssi_5020::overwrite_file(path, sink)?,
        Method::RcmpTssitOpsII => rcmp_tssit_ops_ii::overwrite_file(path, sink)?,
        Method::HmgiS5 => hmgi_s5::overwrite_file(path, sink)?,
        Method::Gutmann => gutmann::overwrite_file(path, sink)?,
        Method::PseudoRandom => pseudo_random::overwrite_file(path, sink)?,
    };
    Ok(())
}

/// Simulates the overwrite of the file at `path` without writing any data,
/// forwarding the same events as [`overwrite_file`] to `sink`.
///
/// Only available when the `dry-run` feature is enabled.
///
/// # Errors
///
/// Returns an error if the execution context cannot be prepared (e.g. the file
/// is inaccessible).
#[cfg(all(not(feature = "error-stack"), feature = "dry-run"))]
pub(crate) fn dry_overwrite_file<S: EventSink>(
    method: &Method,
    path: &Path,
    sink: &mut S,
) -> Result<()> {
    match method {
        Method::Dod522022MECE => dod_522022_mece::dry_overwrite_file(path, sink)?,
        Method::Dod522022ME => dod_522022_me::dry_overwrite_file(path, sink)?,
        Method::Afssi5020 => afssi_5020::dry_overwrite_file(path, sink)?,
        Method::RcmpTssitOpsII => rcmp_tssit_ops_ii::dry_overwrite_file(path, sink)?,
        Method::HmgiS5 => hmgi_s5::dry_overwrite_file(path, sink)?,
        Method::Gutmann => gutmann::dry_overwrite_file(path, sink)?,
        Method::PseudoRandom => pseudo_random::dry_overwrite_file(path, sink)?,
    }
    Ok(())
}

/// Overwrites the file at `path` using the algorithm specified by `method`,
/// forwarding progress events to `sink`.
///
/// This is the single dispatch point used by the executor. Each method
/// submodule owns the concrete pass implementation.
///
/// # Errors
///
/// Returns an [`error_stack::Report`] wrapping [`Error`](crate::Error) if any
/// overwrite pass fails (I/O error or permission denied).
#[cfg(feature = "error-stack")]
pub(crate) fn overwrite_file<S: EventSink>(
    method: &Method,
    path: &Path,
    sink: &mut S,
) -> Result<()> {
    match method {
        Method::Dod522022MECE => dod_522022_mece::overwrite_file(path, sink)?,
        Method::Dod522022ME => dod_522022_me::overwrite_file(path, sink)?,
        Method::Afssi5020 => afssi_5020::overwrite_file(path, sink)?,
        Method::RcmpTssitOpsII => rcmp_tssit_ops_ii::overwrite_file(path, sink)?,
        Method::HmgiS5 => hmgi_s5::overwrite_file(path, sink)?,
        Method::Gutmann => gutmann::overwrite_file(path, sink)?,
        Method::PseudoRandom => pseudo_random::overwrite_file(path, sink)?,
    };
    Ok(())
}

/// Simulates the overwrite of the file at `path` without writing any data,
/// forwarding the same events as [`overwrite_file`] to `sink`.
///
/// Only available when the `dry-run` feature is enabled.
///
/// # Errors
///
/// Returns an [`error_stack::Report`] wrapping [`Error`](crate::Error) if the
/// execution context cannot be prepared.
#[cfg(all(feature = "error-stack", feature = "dry-run"))]
pub(crate) fn dry_overwrite_file<S: EventSink>(
    method: &Method,
    path: &Path,
    sink: &mut S,
) -> Result<()> {
    match method {
        Method::Dod522022MECE => dod_522022_mece::dry_overwrite_file(path, sink)?,
        Method::Dod522022ME => dod_522022_me::dry_overwrite_file(path, sink)?,
        Method::Afssi5020 => afssi_5020::dry_overwrite_file(path, sink)?,
        Method::RcmpTssitOpsII => rcmp_tssit_ops_ii::dry_overwrite_file(path, sink)?,
        Method::HmgiS5 => hmgi_s5::dry_overwrite_file(path, sink)?,
        Method::Gutmann => gutmann::dry_overwrite_file(path, sink)?,
        Method::PseudoRandom => pseudo_random::dry_overwrite_file(path, sink)?,
    }
    Ok(())
}
