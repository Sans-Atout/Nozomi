use crate::Result;
use crate::error::{FSProblem, IoError, PlanningError};
use std::path::{Path, PathBuf};

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
pub(crate) fn execution_plan(root_path: &Path) -> Result<ExecutionPlan> {
    let mut files = Vec::new();
    let mut directories = Vec::new();
    visit(root_path, &mut files, &mut directories)?;
    Ok(ExecutionPlan { files, directories })
}

fn visit(path: &Path, files: &mut Vec<PathBuf>, directories: &mut Vec<PathBuf>) -> Result<()> {
    #[cfg(feature = "secure_log")]
    let md5_value = md5::compute(path.to_string_lossy().to_string());
    if !path.exists() {
        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        error!("[{}]\t did not exist", &path.to_string_lossy().to_string());
        #[cfg(all(feature = "log", feature = "secure_log"))]
        error!("[{:x}]\tdid not exist", md5_value);
        return Err(IoError::Filesystem(
            FSProblem::NotFound,
            path.as_os_str()
                .to_str()
                .ok_or(PlanningError::StringConversion)?
                .to_string(),
        )
        .into());
    }

    if path.is_dir() {
        directories.push(path.to_path_buf());

        let directory_entry = std::fs::read_dir(path).map_err(|_| {
            IoError::Filesystem(
                FSProblem::ReadFolder,
                path.as_os_str()
                    .to_str()
                    .ok_or(PlanningError::StringConversion)
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
                IoError::Filesystem(
                    FSProblem::ReadFolder,
                    path.as_os_str()
                        .to_str()
                        .ok_or(PlanningError::StringConversion)
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
