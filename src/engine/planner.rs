use crate::error::FSProblem;
use std::path::{Path, PathBuf};

// -- Region : feature import
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::{Report, ResultExt};

#[cfg(feature = "log")]
use log::error;

/// An ordered list of filesystem entries to process during a deletion run.
///
/// `files` are overwritten and removed first; `directories` are removed in
/// reverse order afterwards so that children are always cleaned up before
/// their parent.
#[derive(Debug)]
pub(crate) struct ExecutionPlan {
    /// Regular files to overwrite and delete, in discovery order.
    pub files: Vec<PathBuf>,
    /// Directories to remove after all files have been deleted, deepest first.
    pub directories: Vec<PathBuf>,
}

/// Walks the filesystem tree rooted at `root_path` and builds an
/// [`ExecutionPlan`].
///
/// If `root_path` is a regular file, the plan contains only that file. If it
/// is a directory, the function recurses into every subdirectory and collects
/// all files.
///
/// # Errors
///
/// Returns an error if `root_path` does not exist or a directory entry cannot
/// be read.
#[cfg(not(feature = "error-stack"))]
pub(crate) fn execution_plan(root_path: &Path) -> Result<ExecutionPlan> {
    let mut files = Vec::new();
    let mut directories = Vec::new();
    visit(root_path, &mut files, &mut directories)?;
    Ok(ExecutionPlan { files, directories })
}

#[cfg(not(feature = "error-stack"))]
fn visit(path: &Path, files: &mut Vec<PathBuf>, directories: &mut Vec<PathBuf>) -> Result<()> {
    #[cfg(feature = "secure_log")]
    let md5_value = md5::compute(&path.to_string_lossy().to_string());
    if !path.exists() {
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        error!("[{}]\t did not exist", &path.to_string_lossy().to_string());
        #[cfg(all(feature = "log", feature = "secure_log"))]
        error!("[{:x}]\tdid not exist", md5_value);
        return Err(Error::SystemProblem(
            FSProblem::NotFound,
            path.as_os_str()
                .to_str()
                .ok_or(Error::StringConversionError)?
                .to_string(),
        ));
    }

    if path.is_dir() {
        directories.push(path.to_path_buf());

        let directory_entry = std::fs::read_dir(path).map_err(|_| {
            Error::SystemProblem(
                FSProblem::ReadFolder,
                path.as_os_str()
                    .to_str()
                    .ok_or(Error::StringConversionError)
                    .unwrap()
                    .to_string(),
            )
        })?;
        for dir_entry in directory_entry {
            if dir_entry.is_err() {
                #[cfg(all(feature = "log", not(feature = "secure_log")))]
                error!("[{:#?}]\t error during file reading", path);
                #[cfg(all(feature = "log", feature = "secure_log"))]
                error!("[{:x}]\t error during file reading", md5_value);
                continue;
            }
            let entry = dir_entry.map_err(|_| {
                Error::SystemProblem(
                    FSProblem::ReadFolder,
                    path.as_os_str()
                        .to_str()
                        .ok_or(Error::StringConversionError)
                        .unwrap()
                        .to_string(),
                )
            })?;
            visit(&entry.path(), files, directories)?;
        }
        return Ok(());
    }
    files.push(path.to_path_buf());
    Ok(())
}

/// Walks the filesystem tree rooted at `root_path` and builds an
/// [`ExecutionPlan`].
///
/// If `root_path` is a regular file, the plan contains only that file. If it
/// is a directory, the function recurses into every subdirectory and collects
/// all files.
///
/// # Errors
///
/// Returns an [`error_stack::Report`] wrapping [`Error`](crate::Error) if
/// `root_path` does not exist or a directory entry cannot be read.
#[cfg(feature = "error-stack")]
pub(crate) fn execution_plan(root_path: &Path) -> Result<ExecutionPlan> {
    let mut files = Vec::new();
    let mut directories = Vec::new();
    visit(root_path, &mut files, &mut directories)?;
    Ok(ExecutionPlan { files, directories })
}

#[cfg(feature = "error-stack")]
fn visit(path: &Path, files: &mut Vec<PathBuf>, directories: &mut Vec<PathBuf>) -> Result<()> {
    #[cfg(feature = "secure_log")]
    let md5_value = md5::compute(path.to_string_lossy().to_string());

    if !path.exists() {
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        error!("[{}]\t did not exist", &path.to_string_lossy());
        #[cfg(all(feature = "log", feature = "secure_log"))]
        error!(
            "[{:x}]\tdid not exist",
            md5::compute(path.to_string_lossy().to_string())
        );
        return Err(Report::new(Error::SystemProblem(
            FSProblem::NotFound,
            path.as_os_str()
                .to_str()
                .ok_or(Error::StringConversionError)?
                .to_string(),
        )));
    }

    if path.is_dir() {
        directories.push(path.to_path_buf());

        let directory_entry = std::fs::read_dir(path).change_context(Error::SystemProblem(
            FSProblem::ReadFolder,
            path.as_os_str()
                .to_str()
                .ok_or(Error::StringConversionError)
                .unwrap()
                .to_string(),
        ))?;
        for dir_entry in directory_entry {
            if dir_entry.is_err() {
                #[cfg(all(feature = "log", not(feature = "secure_log")))]
                error!("[{:#?}]\t error during file reading", path);
                #[cfg(all(feature = "log", feature = "secure_log"))]
                error!("[{:x}]\t error during file reading", md5_value);
                continue;
            }
            let entry = dir_entry.map_err(|_| {
                Error::SystemProblem(
                    FSProblem::ReadFolder,
                    path.as_os_str()
                        .to_str()
                        .ok_or(Error::StringConversionError)
                        .unwrap()
                        .to_string(),
                )
            })?;
            visit(&entry.path(), files, directories)?;
        }
        return Ok(());
    }

    files.push(path.to_path_buf());
    Ok(())
}
