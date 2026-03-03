use std::path::{Path, PathBuf};

#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

use super::request::{DeleteMethod, DeleteRequest};

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DeleteRequestBuilder {
    path: Option<PathBuf>,
    method: Option<DeleteMethod>,
    #[cfg(feature = "dry-run")]
    dry_run: bool,
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

    #[cfg(feature = "dry-run")]
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }
}

#[cfg(not(feature = "error-stack"))]
impl DeleteRequestBuilder {
    pub fn build(self) -> Result<DeleteRequest> {
        let path = self.path.ok_or(Error::MissingParameter("path"))?;

        let method = self.method.ok_or(Error::MissingParameter("method"))?;

        Ok(DeleteRequest {
            path,
            method,
            #[cfg(feature = "dry-run")]
            dry_run : self.dry_run
        })
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

#[cfg(feature = "error-stack")]
impl DeleteRequestBuilder {
    pub fn build(self) -> Result<DeleteRequest> {
        let path = self.path.ok_or(Error::MissingParameter("path"))?;
        let method = self.method.ok_or(Error::MissingParameter("method"))?;
        Ok(DeleteRequest {
            path,
            method,
            #[cfg(feature = "dry-run")]
            dry_run : self.dry_run.clone(),
        })
    }
}

#[cfg(all(test, feature = "error-stack"))]
mod tests {
    use super::*;
    use crate::Method;
    use crate::api::delete::request::DeleteMethod;
    use error_stack::Frame;
    use std::path::PathBuf;

    #[test]
    fn build_fails_when_path_is_missing() {
        let result = DeleteRequestBuilder::new()
            .method(DeleteMethod::BuiltIn(Method::default()))
            .build();

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert!(err.frames().any(|frame| {
            frame
                .downcast_ref::<Error>()
                .is_some_and(|e| matches!(e, Error::MissingParameter("path")))
        }));
    }

    #[test]
    fn build_fails_when_method_is_missing() {
        use std::path::PathBuf;

        let result = DeleteRequestBuilder::new()
            .path(PathBuf::from("/tmp/file.txt"))
            .build();

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert!(err.frames().any(|frame| {
            frame
                .downcast_ref::<Error>()
                .is_some_and(|e| matches!(e, Error::MissingParameter("method")))
        }));
    }

    #[test]
    fn build_succeeds_when_all_parameters_are_present() {
        use std::path::PathBuf;

        let result = DeleteRequestBuilder::new()
            .path(PathBuf::from("/tmp/file.txt"))
            .method(DeleteMethod::BuiltIn(Method::default()))
            .build();

        assert!(result.is_ok());
    }
}
