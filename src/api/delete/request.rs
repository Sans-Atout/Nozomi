#[cfg(not(feature = "error-stack"))]
use crate::Result;
#[cfg(feature = "error-stack")]
use crate::Result;
use crate::api::delete::{DeleteReport, DeleteRequestBuilder};
use crate::engine;
#[cfg(feature = "dry-run")]
use crate::engine::emit_safe;
use crate::{DeleteEvent, EventSink, Method};
use std::path::PathBuf;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DeleteRequest {
    pub(crate) path: PathBuf,
    pub(crate) method: DeleteMethod,
    #[cfg(feature = "dry-run")]
    pub(crate) dry_run: bool,
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
        let mut sink = NoopSink;
        self.run_with(&mut sink)
    }

    pub fn run_with<S: EventSink>(&self, sink: &mut S) -> Result<DeleteReport> {
        #[cfg(feature = "dry-run")]
        if self.dry_run {
            emit_safe(
                sink,
                DeleteEvent::DryRunStarted {
                    path: self.path.to_path_buf(),
                },
            );
            match &self.method {
                DeleteMethod::BuiltIn(method) => {
                    engine::dry_run(method, &self.path, sink)?;
                    emit_safe(
                        sink,
                        DeleteEvent::DryRunCompleted {
                            path: self.path.to_path_buf(),
                        },
                    );
                    return Ok(DeleteReport {
                        path: self.path.clone(),
                        method: *method,
                    });
                }
            }
        }

        match &self.method {
            DeleteMethod::BuiltIn(method) => {
                engine::run(method, &self.path, sink)?;
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
        let mut sink = NoopSink;
        self.run_with(&mut sink)
    }

    pub fn run_with<S: EventSink>(&self, sink: &mut S) -> Result<DeleteReport> {
        #[cfg(feature = "dry-run")]
        if self.dry_run {
            emit_safe(
                sink,
                DeleteEvent::DryRunStarted {
                    path: self.path.to_path_buf(),
                },
            );
            match &self.method {
                DeleteMethod::BuiltIn(method) => {
                    engine::dry_run(method, &self.path, sink)?;
                    emit_safe(
                        sink,
                        DeleteEvent::DryRunCompleted {
                            path: self.path.to_path_buf(),
                        },
                    );
                    return Ok(DeleteReport {
                        path: self.path.clone(),
                        method: *method,
                    });
                }
            }
        }

        match &self.method {
            DeleteMethod::BuiltIn(method) => {
                engine::run(method, &self.path, sink)?;
                Ok(DeleteReport {
                    path: self.path.clone(),
                    method: *method,
                })
            }
        }
    }
}

pub(crate) struct NoopSink;

impl EventSink for NoopSink {
    fn emit(&mut self, _: DeleteEvent) {}
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
