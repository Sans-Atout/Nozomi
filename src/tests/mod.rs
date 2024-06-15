#[cfg(feature = "error-stack")]
pub mod enhanced;

#[cfg(not(feature = "error-stack"))]
pub mod standard;

use crate::{error::FSProblem, models::SecureDelete, Method};
use std::fmt::Debug;

/// Function to test if an type (struct/enum) as send and sync trait
fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}

/// Function to test if an type (struct/enum) as send and sync trait
fn has_needed<T: Clone + Debug>() {}

#[test]
/// This test is used to check whether enums and structures have the various basic traits.
/// If this test fails, the library cannot be compiled
fn validate_structure() {
    // -- Secure Delete struct
    has_needed::<SecureDelete>();
    has_send_sync::<SecureDelete>();
    // -- Test Type enum
    has_send_sync::<TestType>();
    has_needed::<TestType>();
    // -- File System possible Problem enum
    has_send_sync::<FSProblem>();
    has_needed::<FSProblem>();
    // -- overwrite Method enum
    has_send_sync::<Method>();
    has_needed::<Method>();

    // -- error method
    #[cfg(not(feature = "error-stack"))]
    has_send_sync::<crate::Error>();
    #[cfg(feature = "error-stack")]
    has_send_sync::<crate::Error>();
    #[cfg(feature = "error-stack")]
    has_needed::<crate::Error>();
}

/// Basic lorem ipsum text used to create file
const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent congue diam risus, quis hendrerit nunc commodo id. Duis volutpat vitae leo at malesuada. Phasellus libero nisl, auctor sit amet dapibus eget, egestas eget velit. Curabitur fermentum, libero vel dictum hendrerit, metus leo pellentesque tortor, et vehicula nisl felis elementum purus. Etiam eu ex in odio tincidunt maximus. Ut luctus blandit ligula et dignissim. Duis id imperdiet urna. Sed porttitor nulla vitae sollicitudin sagittis. Donec ut justo ut risus pretium dignissim. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec eu rhoncus erat, sed sagittis elit. Proin dui nisl, varius nec volutpat sed, lobortis nec tellus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.\n";

/// Enum that describe all tests' type used to test global method
#[derive(Debug, Clone, Copy)]
pub enum TestType {
    /// Test deleting a 1KB file
    SmallFile,
    /// Test deleting a 1MB file
    MediumFile,
    /// Test deleting a 10MB file
    LargeFile,
    /// Test deleting a read only file
    WritingError,
    /// Test deleting a folder containing multiple files
    Folder,
    /// Test the overwriting function
    OverwriteOnly,
    #[cfg(feature = "log")]
    /// Test deleting a 1KB file with log feature activated
    LogMini,
    #[cfg(feature = "secure_log")]
    /// Test deleting a 1KB file with secure log feature activated
    SecureLog,
}
