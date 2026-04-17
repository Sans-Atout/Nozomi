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

/// A fully validated request to securely delete a file.
///
/// Instances are constructed exclusively through [`DeleteRequestBuilder`] to
/// guarantee that all required fields are present before execution begins.
/// Once built, a request is immutable and can be executed via [`run`] or
/// [`run_with`].
///
/// [`run`]: DeleteRequest::run
/// [`run_with`]: DeleteRequest::run_with
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DeleteRequest {
    pub(crate) path: PathBuf,
    pub(crate) method: DeleteMethod,
    #[cfg(feature = "dry-run")]
    pub(crate) dry_run: bool,
}

/// Specifies the overwrite strategy to apply during secure deletion.
///
/// Currently the only variant is [`BuiltIn`](DeleteMethod::BuiltIn), which
/// delegates to one of the standards defined in [`Method`]. Additional
/// variants (e.g., custom strategies) may be added in future releases.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DeleteMethod {
    /// One of the built-in, standards-compliant overwrite methods.
    BuiltIn(Method),
}

impl DeleteRequest {
    /// Creates a new [`DeleteRequestBuilder`] as the entry point for
    /// constructing a `DeleteRequest`.
    pub fn builder() -> DeleteRequestBuilder {
        DeleteRequestBuilder::new()
    }
}

#[cfg(not(feature = "error-stack"))]
impl DeleteRequest {
    /// Executes the secure deletion pipeline and returns a [`DeleteReport`] on success.
    ///
    /// This is the zero-overhead convenience wrapper around [`run_with`]. It discards
    /// all progress events. Use [`run_with`] instead if you need to observe or log
    /// individual deletion steps.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the target path cannot be opened, overwritten,
    /// or removed.
    ///
    /// [`run_with`]: DeleteRequest::run_with
    pub fn run(&self) -> Result<DeleteReport> {
        let mut sink = NoopSink;
        self.run_with(&mut sink)
    }

    /// Executes the secure deletion pipeline, forwarding structured events to `sink`.
    ///
    /// Each step of the overwrite process emits a [`DeleteEvent`] through the
    /// provided [`EventSink`], enabling progress tracking, audit logging, or
    /// integration with external monitoring systems.
    ///
    /// When the `dry-run` feature is enabled and [`dry_run`](super::builder::DeleteRequestBuilder::dry_run)
    /// is set to `true`, the pipeline is simulated: events are emitted but no
    /// data is written to disk.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the target path cannot be opened, overwritten,
    /// or removed.
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
    /// Executes the secure deletion pipeline and returns a [`DeleteReport`] on success.
    ///
    /// This is the zero-overhead convenience wrapper around [`run_with`]. It discards
    /// all progress events. Use [`run_with`] instead if you need to observe or log
    /// individual deletion steps.
    ///
    /// # Errors
    ///
    /// Returns an [`error_stack::Report`] wrapping [`Error`] if the target path
    /// cannot be opened, overwritten, or removed.
    ///
    /// [`run_with`]: DeleteRequest::run_with
    pub fn run(&self) -> Result<DeleteReport> {
        let mut sink = NoopSink;
        self.run_with(&mut sink)
    }

    /// Executes the secure deletion pipeline, forwarding structured events to `sink`.
    ///
    /// Each step of the overwrite process emits a [`DeleteEvent`] through the
    /// provided [`EventSink`], enabling progress tracking, audit logging, or
    /// integration with external monitoring systems.
    ///
    /// When the `dry-run` feature is enabled and [`dry_run`](super::builder::DeleteRequestBuilder::dry_run)
    /// is set to `true`, the pipeline is simulated: events are emitted but no
    /// data is written to disk.
    ///
    /// # Errors
    ///
    /// Returns an [`error_stack::Report`] wrapping [`Error`] if the target path
    /// cannot be opened, overwritten, or removed.
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

/// An [`EventSink`] that silently discards every emitted event.
///
/// Used internally by [`DeleteRequest::run`] to avoid requiring callers to
/// supply a sink when event observation is not needed.
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
