use crate::Method;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DeleteReport {
    pub path: PathBuf,
    pub method: Method,
}
