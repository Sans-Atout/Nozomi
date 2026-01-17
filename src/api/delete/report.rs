use std::path::PathBuf;
use crate::Method;

#[derive(Debug, Clone)]
pub struct DeleteReport {
	pub path: PathBuf,
	pub method: Method,
}
