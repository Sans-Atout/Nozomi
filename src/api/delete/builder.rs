use std::path::{Path, PathBuf};

#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};


use super::request::{DeleteMethod, DeleteRequest};

#[derive(Debug, Default)]
#[cfg_attr(test,derive(PartialEq))]
pub struct DeleteRequestBuilder {
	path: Option<PathBuf>,
	method: Option<DeleteMethod>,
}

impl DeleteRequestBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
		self.path = Some(path.as_ref().to_path_buf());
		self
	}

	pub fn method(mut self, method: DeleteMethod) -> Self {
		self.method = Some(method);
		self
	}

}

#[cfg(not(feature = "error-stack"))]
impl DeleteRequestBuilder {
	pub fn build(self) -> Result<DeleteRequest> {
		let path = self.path.ok_or(
			Error::MissingParameter("path")
		)?;

		let method = self.method.ok_or(
			Error::MissingParameter("method")
		)?;

		Ok(DeleteRequest { path, method })
	}
}

#[cfg(all(test, not(feature = "error-stack")))]
mod tests {
	use super::*;
	use crate::{Error, Method};
	use pretty_assertions::assert_eq;
	use std::path::PathBuf;

	#[test]
	fn build_fails_when_path_is_missing() {
		let result = DeleteRequestBuilder::new()
			.method(DeleteMethod::BuiltIn(Method::default()))
			.build();

		let err = result.expect_err("builder should fail without path");

		assert_eq!(err, Error::MissingParameter("path"));
	}

	#[test]
	fn build_fails_when_method_is_missing() {
		let result = DeleteRequestBuilder::new()
			.path(PathBuf::from("/tmp/file.txt"))
			.build();

		let err = result.expect_err("builder should fail without method");

		assert_eq!(err, Error::MissingParameter("method"));
	}

	#[test]
	fn build_succeeds_when_all_parameters_are_present() {
		let result = DeleteRequestBuilder::new()
			.path(PathBuf::from("/tmp/file.txt"))
			.method(DeleteMethod::BuiltIn(Method::default()))
			.build();

		assert!(result.is_ok());
	}
}