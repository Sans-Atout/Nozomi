#[cfg(feature="error-stack")]
pub mod enhanced;

#[cfg(feature="error-stack")]
pub use enhanced as utils;

#[cfg(not(feature="error-stack"))]
pub mod standard;

use std::fmt::Debug;

use crate::models::SecureDelete;

fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}
fn has_needed<T: Clone + Debug>() {}

#[test]
fn validate_structure(){
    has_needed::<SecureDelete>();
    has_send_sync::<SecureDelete>();
}