use crate::Method;
use crate::api::delete::{DeleteReport, DeleteRequestBuilder};
use crate::engine;
use std::path::PathBuf;

#[cfg(not(feature = "error-stack"))]
use crate::Result;
#[cfg(feature = "error-stack")]
use crate::Result;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DeleteRequest {
    pub(crate) path: PathBuf,
    pub(crate) method: DeleteMethod,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DeleteMethod {
    BuiltIn(Method),
}

impl DeleteRequest {
    pub fn builder() -> DeleteRequestBuilder {
        DeleteRequestBuilder::new()
    }
}

#[cfg(not(feature = "error-stack"))]
impl DeleteRequest {
    pub fn run(&self) -> Result<DeleteReport> {
        match &self.method {
            DeleteMethod::BuiltIn(method) => {
                engine::run(method, &self.path)?;
                Ok(DeleteReport {
                    path: self.path.clone(),
                    method: *method,
                })
            }
        }
    }
}

#[cfg(feature = "error-stack")]
impl DeleteRequest {
    pub fn run(&self) -> Result<DeleteReport> {
        match &self.method {
            DeleteMethod::BuiltIn(method) => {
                engine::run(method, &self.path)?;
                Ok(DeleteReport {
                    path: self.path.clone(),
                    method: *method,
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::DeleteRequest;
    use crate::api::delete::DeleteRequestBuilder;
    use pretty_assertions::assert_eq;

    #[test]
    fn delete_request() {
        let req_builder = DeleteRequest::builder();
        let delete_builder = DeleteRequestBuilder::new();
        assert_eq!(req_builder, delete_builder);
    }
}
