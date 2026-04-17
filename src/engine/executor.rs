use crate::{DeleteEvent, EventSink, Method};
use std::path::Path;

use super::overwrite;
use super::planner;
use crate::engine::utils::{delete_dir, delete_file, emit_safe};

#[cfg(not(feature = "error-stack"))]
use crate::Result;

#[cfg(feature = "error-stack")]
use crate::Result;

/// Executes the full secure deletion pipeline for the entry at `path`.
///
/// The engine builds an [`ExecutionPlan`](super::planner::ExecutionPlan), overwrites
/// every file in the plan using the chosen `method`, deletes each file, and
/// finally removes empty directories in reverse order. Progress events are
/// forwarded to `sink` throughout.
///
/// # Errors
///
/// Returns an error if any overwrite pass, file deletion, or directory removal fails.
#[cfg(not(feature = "error-stack"))]
pub(crate) fn run<S: EventSink>(method: &Method, path: &Path, sink: &mut S) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::DeletionStarted {
            path: path.to_path_buf(),
        },
    );
    let result = (|| {
        let plan = planner::execution_plan(path)?;

        for file_path in &plan.files {
            overwrite::overwrite_file(method, file_path, sink)?;
        }

        for file_path in &plan.files {
            delete_file(file_path)?;
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: file_path.clone(),
                },
            );
        }

        for dir_path in plan.directories.iter().rev() {
            delete_dir(dir_path)?;
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: dir_path.clone(),
                },
            );
        }

        Ok(())
    })();

    emit_safe(
        sink,
        DeleteEvent::DeletionFinished {
            path: path.to_path_buf(),
        },
    );

    result
}

/// Simulates the secure deletion pipeline without performing any write or
/// delete operations on disk.
///
/// The engine builds an execution plan and emits the same events as [`run`],
/// but no bytes are written and no files are removed. Useful for verifying
/// configuration and observing expected event sequences.
///
/// Only available when the `dry-run` feature is enabled.
///
/// # Errors
///
/// Returns an error if the execution plan cannot be built (e.g. path not found).
#[cfg(all(feature = "dry-run", not(feature = "error-stack")))]
pub(crate) fn dry_run<S: EventSink>(method: &Method, path: &Path, sink: &mut S) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::DeletionStarted {
            path: path.to_path_buf(),
        },
    );

    let result = (|| {
        let plan = planner::execution_plan(path)?;

        for file_path in &plan.files {
            overwrite::dry_overwrite_file(method, file_path, sink)?;
        }
        for file_path in &plan.files {
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: file_path.clone(),
                },
            );
        }

        for dir_path in plan.directories.iter().rev() {
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: dir_path.clone(),
                },
            );
        }
        Ok(())
    })();

    emit_safe(
        sink,
        DeleteEvent::DeletionFinished {
            path: path.to_path_buf(),
        },
    );
    result
}

/// Executes the full secure deletion pipeline for the entry at `path`.
///
/// The engine builds an [`ExecutionPlan`](super::planner::ExecutionPlan), overwrites
/// every file in the plan using the chosen `method`, deletes each file, and
/// finally removes empty directories in reverse order. Progress events are
/// forwarded to `sink` throughout.
///
/// # Errors
///
/// Returns an [`error_stack::Report`] wrapping [`Error`](crate::Error) if any
/// overwrite pass, file deletion, or directory removal fails.
#[cfg(feature = "error-stack")]
pub(crate) fn run<S: EventSink>(method: &Method, path: &Path, sink: &mut S) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::DeletionStarted {
            path: path.to_path_buf(),
        },
    );

    let result = (|| {
        let plan = planner::execution_plan(path)?;

        for file_path in &plan.files {
            overwrite::overwrite_file(method, file_path, sink)?;
        }

        for file_path in &plan.files {
            delete_file(file_path)?;
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: file_path.clone(),
                },
            );
        }

        for dir_path in plan.directories.iter().rev() {
            delete_dir(dir_path)?;
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: dir_path.clone(),
                },
            );
        }
        Ok(())
    })();

    emit_safe(
        sink,
        DeleteEvent::DeletionFinished {
            path: path.to_path_buf(),
        },
    );

    result
}

/// Simulates the secure deletion pipeline without performing any write or
/// delete operations on disk.
///
/// The engine builds an execution plan and emits the same events as [`run`],
/// but no bytes are written and no files are removed. Useful for verifying
/// configuration and observing expected event sequences.
///
/// Only available when the `dry-run` feature is enabled.
///
/// # Errors
///
/// Returns an [`error_stack::Report`] wrapping [`Error`](crate::Error) if the
/// execution plan cannot be built (e.g. path not found).
#[cfg(all(feature = "dry-run", feature = "error-stack"))]
pub(crate) fn dry_run<S: EventSink>(method: &Method, path: &Path, sink: &mut S) -> Result<()> {
    emit_safe(
        sink,
        DeleteEvent::DeletionStarted {
            path: path.to_path_buf(),
        },
    );

    let result = (|| {
        let plan = planner::execution_plan(path)?;

        for file_path in &plan.files {
            overwrite::dry_overwrite_file(method, file_path, sink)?;
        }
        for file_path in &plan.files {
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: file_path.clone(),
                },
            );
        }

        for dir_path in plan.directories.iter().rev() {
            emit_safe(
                sink,
                DeleteEvent::EntryDeleted {
                    path: dir_path.clone(),
                },
            );
        }
        Ok(())
    })();

    emit_safe(
        sink,
        DeleteEvent::DeletionFinished {
            path: path.to_path_buf(),
        },
    );
    result
}
