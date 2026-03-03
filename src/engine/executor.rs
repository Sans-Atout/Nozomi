use crate::{DeleteEvent, EventSink, Method};
use std::path::Path;

use super::overwrite;
use super::planner;
use crate::engine::utils::{delete_dir, delete_file, emit_safe};

#[cfg(not(feature = "error-stack"))]
use crate::Result;

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

#[cfg(feature = "log")]
use log::info;

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
