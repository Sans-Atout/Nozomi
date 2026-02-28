use std::path::PathBuf;

/// Structured execution events emitted during deletion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeleteEvent {
	/// Deletion process has started for a given path
	DeletionStarted {
		path: PathBuf,
	},
	/// A filesystem entry has been successfully overwritten
	EntryOverwritePass {
		path: PathBuf,
		pass: u32,
		total_passes: u32,
	},

	/// A filesystem entry has been successfully deleted
	EntryDeleted {
		path: PathBuf,
	},

	/// Deletion process has finished
	DeletionFinished {
		path: PathBuf,
	},
}

/// Sink for deletion execution events.
///
/// Any failure inside the sink must not affect deletion execution.
pub trait EventSink {
	/// Emit a structured deletion event.
	///
	/// This method must never influence control flow.
	/// Panics inside implementations will be caught by the engine.
	fn emit(&mut self, event: DeleteEvent);
}