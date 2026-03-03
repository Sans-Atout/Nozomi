//! Internal filesystem-related helpers.
//!
//! This module contains low-level helpers mirroring legacy filesystem
//! operations. It must not contain business logic or orchestration.

#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};
use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;

use crate::error::FSProblem;
use crate::{DeleteEvent, EventSink, SecureDelete};
#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::{Context, Report, ResultExt};
#[cfg(feature = "log")]
use log::trace;
use rand::RngCore;

pub(crate) fn generate_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    rand::rng().fill_bytes(&mut seed);
    seed
}

#[cfg(feature = "verify")]
pub(super) fn get_legacy_buffer(pattern: &[u8; 3], size: usize) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    for i in 0..size {
        let pattern_index = i % 3;
        buffer.push(pattern[pattern_index]);
    }
    buffer
}

#[cfg(not(feature = "error-stack"))]
pub(crate) fn delete_file(path: &Path) -> Result<()> {
    #[cfg(feature = "secure_log")]
    let md5_value = md5::compute(&path.to_string_lossy().to_string());
    #[cfg(feature = "log")]
    trace!("[{}]\tBeginning of deletion", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!("[{:x}]\tBeginning of deletion", &md5_value);
    let zero_name = zero_name(path)?;
    let mut old_path = path.to_path_buf();
    let mut new_path = path.to_path_buf();
    new_path.set_file_name(&zero_name);
    fs::rename(&old_path, &new_path).map_err(|_| {
        Error::SystemProblem(FSProblem::Rename, format!("{}", path.to_string_lossy()))
    })?;
    old_path.set_file_name(&zero_name);
    #[cfg(feature = "log")]
    trace!("[{}]\tRenaming", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!(
        "[{:x}]\tRenaming to {:x}",
        &md5_value,
        md5::compute(&new_path.to_string_lossy().to_string())
    );

    let mut anon_file_size = zero_name.len();
    while anon_file_size > 1 {
        anon_file_size -= 1;
        let new_file_name = (0..anon_file_size).map(|_| "0").collect::<String>();
        new_path.set_file_name(&new_file_name);
        fs::rename(&old_path, &new_path).map_err(|_| {
            Error::SystemProblem(FSProblem::Rename, format!("{}", path.to_string_lossy()))
        })?;
        old_path.set_file_name(&new_file_name);
        #[cfg(feature = "log")]
        trace!("[{}]\tRenaming", &path.to_string_lossy());
        #[cfg(feature = "secure_log")]
        trace!(
            "[{:x}]\tRenaming to {:x}",
            &md5_value,
            md5::compute(&new_path.to_string_lossy().to_string())
        );
    }
    fs::remove_file(&new_path).map_err(|_| {
        Error::SystemProblem(
            FSProblem::Delete,
            format!("{}", &new_path.to_string_lossy()),
        )
    })
}

pub(super) fn emit_safe<S: EventSink>(sink: &mut S, event: DeleteEvent) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        sink.emit(event);
    }));
}

#[cfg(not(feature = "error-stack"))]
pub(crate) fn delete_dir(path: &Path) -> Result<()> {
    #[cfg(feature = "secure_log")]
    let md5_value = md5::compute(&path.to_string_lossy().to_string());
    #[cfg(feature = "log")]
    trace!("[{}]\tBeginning of deletion", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!("[{:x}]\tBeginning of deletion", &md5_value);
    let zero_name = zero_name(path)?;
    let mut old_path = path.to_path_buf();
    let mut new_path = path.to_path_buf();
    new_path.set_file_name(&zero_name);
    fs::rename(&old_path, &new_path).map_err(|_| {
        Error::SystemProblem(FSProblem::Rename, format!("{}", path.to_string_lossy()))
    })?;
    old_path.set_file_name(&zero_name);
    #[cfg(feature = "log")]
    trace!("[{}]\tRenaming", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!(
        "[{:x}]\tRenaming to {:x}",
        &md5_value,
        md5::compute(&new_path.to_string_lossy().to_string())
    );

    let mut anon_file_size = zero_name.len();
    while anon_file_size > 1 {
        anon_file_size -= 1;
        let new_file_name = (0..anon_file_size).map(|_| "0").collect::<String>();
        new_path.set_file_name(&new_file_name);
        fs::rename(&old_path, &new_path).map_err(|_| {
            Error::SystemProblem(FSProblem::Rename, format!("{}", path.to_string_lossy()))
        })?;
        old_path.set_file_name(&new_file_name);
        #[cfg(feature = "log")]
        trace!("[{}]\tRenaming", &path.to_string_lossy());
        #[cfg(feature = "secure_log")]
        trace!(
            "[{:x}]\tRenaming to {:x}",
            &md5_value,
            md5::compute(&new_path.to_string_lossy().to_string())
        );
    }
    fs::remove_dir(&new_path).map_err(|_| {
        Error::SystemProblem(
            FSProblem::Delete,
            format!("{}", &new_path.to_string_lossy()),
        )
    })
}

#[cfg(not(feature = "error-stack"))]
fn zero_name(path: &Path) -> Result<String> {
    let name = path.file_name().ok_or(Error::NoFileName(SecureDelete::new(
        path.to_str().unwrap(),
    )?))?;
    let new_name = (0..name.len()).map(|_| "0").collect::<String>();
    Ok(new_name)
}

#[cfg(feature = "error-stack")]
pub(crate) fn delete_file(path: &Path) -> Result<()> {
    #[cfg(feature = "secure_log")]
    let md5_value = md5::compute(&path.to_string_lossy().to_string());
    #[cfg(feature = "log")]
    trace!("[{}]\tBeginning of deletion", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!("[{:x}]\tBeginning of deletion", &md5_value);
    let zero_name = zero_name(path)?;
    let mut old_path = path.to_path_buf();
    let mut new_path = path.to_path_buf();
    new_path.set_file_name(&zero_name);
    fs::rename(&old_path, &new_path).change_context(Error::SystemProblem(
        FSProblem::Rename,
        format!("{}", path.to_string_lossy()),
    ))?;
    old_path.set_file_name(&zero_name);
    #[cfg(feature = "log")]
    trace!("[{}]\tRenaming", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!(
        "[{:x}]\tRenaming to {:x}",
        &md5_value,
        md5::compute(&new_path.to_string_lossy().to_string())
    );

    let mut anon_file_size = zero_name.len();
    while anon_file_size > 1 {
        anon_file_size -= 1;
        let new_file_name = (0..anon_file_size).map(|_| "0").collect::<String>();
        new_path.set_file_name(&new_file_name);
        fs::rename(&old_path, &new_path).change_context(Error::SystemProblem(
            FSProblem::Rename,
            format!("{}", path.to_string_lossy()),
        ))?;
        old_path.set_file_name(&new_file_name);
        #[cfg(feature = "log")]
        trace!("[{}]\tRenaming", &path.to_string_lossy());
        #[cfg(feature = "secure_log")]
        trace!(
            "[{:x}]\tRenaming to {:x}",
            &md5_value,
            md5::compute(&new_path.to_string_lossy().to_string())
        );
    }
    fs::remove_file(&new_path).change_context(Error::SystemProblem(
        FSProblem::Delete,
        format!("{}", &new_path.to_string_lossy()),
    ))
}

#[cfg(feature = "error-stack")]
pub(crate) fn delete_dir(path: &Path) -> Result<()> {
    #[cfg(feature = "secure_log")]
    let md5_value = md5::compute(&path.to_string_lossy().to_string());
    #[cfg(feature = "log")]
    trace!("[{}]\tBeginning of deletion", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!("[{:x}]\tBeginning of deletion", &md5_value);
    let zero_name = zero_name(path)?;
    let mut old_path = path.to_path_buf();
    let mut new_path = path.to_path_buf();
    new_path.set_file_name(&zero_name);
    fs::rename(&old_path, &new_path).change_context(Error::SystemProblem(
        FSProblem::Rename,
        format!("{}", path.to_string_lossy()),
    ))?;
    old_path.set_file_name(&zero_name);
    #[cfg(feature = "log")]
    trace!("[{}]\tRenaming", &path.to_string_lossy());
    #[cfg(feature = "secure_log")]
    trace!(
        "[{:x}]\tRenaming to {:x}",
        &md5_value,
        md5::compute(&new_path.to_string_lossy().to_string())
    );

    let mut anon_file_size = zero_name.len();
    while anon_file_size > 1 {
        anon_file_size -= 1;
        let new_file_name = (0..anon_file_size).map(|_| "0").collect::<String>();
        new_path.set_file_name(&new_file_name);
        fs::rename(&old_path, &new_path).change_context(Error::SystemProblem(
            FSProblem::Rename,
            format!("{}", path.to_string_lossy()),
        ))?;
        old_path.set_file_name(&new_file_name);
        #[cfg(feature = "log")]
        trace!("[{}]\tRenaming", &path.to_string_lossy());
        #[cfg(feature = "secure_log")]
        trace!(
            "[{:x}]\tRenaming to {:x}",
            &md5_value,
            md5::compute(&new_path.to_string_lossy().to_string())
        );
    }
    fs::remove_dir(&new_path).change_context(Error::SystemProblem(
        FSProblem::Delete,
        format!("{}", &new_path.to_string_lossy()),
    ))
}

#[cfg(feature = "error-stack")]
fn zero_name(path: &Path) -> Result<String> {
    let name = path.file_name().ok_or(Error::NoFileName(SecureDelete::new(
        path.to_str().unwrap(),
    )?))?;
    let new_name = (0..name.len()).map(|_| "0").collect::<String>();
    Ok(new_name)
}
