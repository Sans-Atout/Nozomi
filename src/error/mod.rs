#[cfg(not(feature = "error-stack"))]
pub mod standard;

#[cfg(feature = "error-stack")]
pub mod enhanced;

/// Enum that represent different kind of file system problem that Nozomi lib can encounter
#[derive(Debug, Clone, Copy)]
pub enum FSProblem {
    /// Problem during rename process
    Rename,
    /// Problem during file opening operation
    Opening,
    /// Problem during writing process
    Write,
    /// Problem during deletion process
    Delete,
    /// Problem during file enumeration process
    ReadFolder,
    /// Can not found a file or a folder
    NotFound,
    /// Problem with file/folder process
    Permissions,
}

/// Implementing display trait for FSProblem enum
impl core::fmt::Display for FSProblem {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            FSProblem::Rename => write!(fmt, "Rename"),
            FSProblem::Opening => write!(fmt, "Opening"),
            FSProblem::Write => write!(fmt, "writing"),
            FSProblem::Delete => write!(fmt, "Delete"),
            FSProblem::ReadFolder => write!(fmt, "Read Folder"),
            FSProblem::NotFound => write!(fmt, "File/Folder not found"),
            FSProblem::Permissions => write!(fmt, "Change permission error"),
        }
    }
}

#[cfg(test)]
pub (crate) fn rfc1236<T: core::error::Error + Send + Sync + 'static >(){}
