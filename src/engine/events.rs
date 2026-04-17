use std::path::PathBuf;

/// Structured execution events emitted during a deletion run.
///
/// Implementations of [`EventSink`] receive these events in order as the
/// engine progresses. The sequence for a normal (non-dry-run) deletion is:
///
/// 1. [`DeletionStarted`](DeleteEvent::DeletionStarted)
/// 2. [`EntryOverwritePass`](DeleteEvent::EntryOverwritePass) × N (once per pass, per file)
/// 3. [`EntryDeleted`](DeleteEvent::EntryDeleted) × M (once per file, then once per directory)
/// 4. [`DeletionFinished`](DeleteEvent::DeletionFinished)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeleteEvent {
    /// The deletion pipeline has started for the entry at `path`.
    DeletionStarted { path: PathBuf },

    /// A single overwrite pass has completed for the entry at `path`.
    EntryOverwritePass {
        /// Path of the file being overwritten.
        path: PathBuf,
        /// 1-based index of the pass that just completed.
        pass: u32,
        /// Total number of passes scheduled for this file.
        total_passes: u32,
    },

    /// A file or directory has been successfully removed from the filesystem.
    EntryDeleted { path: PathBuf },

    /// The deletion pipeline has finished for the entry at `path`.
    ///
    /// This event is always emitted, even if an earlier step failed, so
    /// consumers can reliably use it as a completion signal.
    DeletionFinished { path: PathBuf },

    /// Post-overwrite byte verification has started for the entry at `path`.
    ///
    /// Only emitted when the `verify` feature is enabled.
    #[cfg(feature = "verify")]
    VerificationStarted { path: PathBuf },

    /// Post-overwrite byte verification succeeded for the entry at `path`.
    ///
    /// Only emitted when the `verify` feature is enabled.
    #[cfg(feature = "verify")]
    VerificationCompleted { path: PathBuf },

    /// Post-overwrite byte verification detected an unexpected byte at `offset`.
    ///
    /// Only emitted when the `verify` feature is enabled.
    #[cfg(feature = "verify")]
    VerificationFailed {
        /// Path of the file whose verification failed.
        path: PathBuf,
        /// Byte offset of the first mismatched byte within the file.
        offset: u64,
    },

    /// A dry-run deletion simulation has started for the entry at `path`.
    ///
    /// Only emitted when the `dry-run` feature is enabled.
    #[cfg(feature = "dry-run")]
    DryRunStarted { path: PathBuf },

    /// A dry-run deletion simulation has completed for the entry at `path`.
    ///
    /// Only emitted when the `dry-run` feature is enabled.
    #[cfg(feature = "dry-run")]
    DryRunCompleted { path: PathBuf },
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
