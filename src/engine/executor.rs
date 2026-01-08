use crate::Method;
use std::path::Path;

use super::overwrite;
use super::planner;
use crate::engine::utils::{delete_dir, delete_file};

#[cfg(not(feature = "error-stack"))]
use crate::Result;

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

#[cfg(feature = "log")]
use log::info;

#[cfg(not(feature = "error-stack"))]
pub(crate) fn run(method: &Method, path: &Path) -> Result<()> {
    let plan = planner::execution_plan(path)?;

    for file_path in &plan.files {
        overwrite::overwrite_file(method, file_path)?;
    }

    for file_path in &plan.files {
        delete_file(file_path)?;
    }

    for dir_path in plan.directories.iter().rev() {
        delete_dir(dir_path)?;
    }
    Ok(())
}

#[cfg(feature = "error-stack")]
pub(crate) fn run(method: &Method, path: &Path) -> Result<()> {
    let plan = planner::execution_plan(path)?;

    for file_path in &plan.files {
        overwrite::overwrite_file(method, file_path)?;
    }

    for file_path in &plan.files {
        delete_file(file_path)?;
    }

    for dir_path in plan.directories.iter().rev() {
        delete_dir(dir_path)?;
    }
    Ok(())
}
