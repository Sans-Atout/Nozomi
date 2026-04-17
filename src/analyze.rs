/// Describes the type of data written during a single overwrite pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassKind {
    /// The entire pass is filled with `0x00` bytes.
    Zero,
    /// The entire pass is filled with `0xFF` bytes.
    One,
    /// The entire pass repeats a single byte value.
    Pattern(u8),
    /// The entire pass repeats a 3-byte pattern cyclically.
    ThreeBytePattern([u8; 3]),
    /// The entire pass is filled with cryptographically random bytes.
    Random,
}

/// Metadata describing a single overwrite pass within a [`Method`](crate::Method).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassInfo {
    /// The type of data written during this pass.
    pub kind: PassKind,
}

/// A static description of the overwrite schedule applied by a [`Method`](crate::Method).
///
/// Obtained via [`Method::analyze`](crate::Method::analyze) when the `analyze` feature is
/// enabled. Useful for auditing or displaying the pass plan to the user before
/// executing a deletion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisReport {
    /// Total number of overwrite passes that will be performed.
    pub pass_count: usize,
    /// Ordered list of pass descriptors, one entry per pass.
    pub passes: Vec<PassInfo>,
}
