use crate::Method;
use std::path::PathBuf;

/// A summary of a completed secure deletion operation.
///
/// `DeleteReport` is returned by [`DeleteRequest::run`](super::request::DeleteRequest::run)
/// and [`DeleteRequest::run_with`](super::request::DeleteRequest::run_with) upon success.
/// It records which file was targeted and which sanitisation standard was applied,
/// making it suitable for audit logging.
#[derive(Debug, Clone)]
pub struct DeleteReport {
    /// Absolute path of the file that was securely deleted.
    pub path: PathBuf,
    /// Overwrite method that was applied during the deletion.
    pub method: Method,
}
