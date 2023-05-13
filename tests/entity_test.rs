use std::fmt::Debug;

use nozomi::{
    error::{InputError, ProcessError},
    OverwriteMethod,
};

fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}
fn has_needed<T: Copy + Clone + Debug>() {}

/// Test if all Entity int this library has Send and Sync trait implemented
///
/// This test is successful if all entity have the Sync and Send trait.
/// If the library compile, then this test must pass (has Send and Sync
///  traits is checked at compile time).
#[test]
fn has_enum_sync_send() {
    has_send_sync::<OverwriteMethod>();
    has_send_sync::<InputError>();
    has_send_sync::<ProcessError>();
}

/// Test if all Entity int this library has Send and Sync trait implemented
///
/// This test is successful if all entity have Copy, Clone and Debug trait
/// If the library compile, then this test must pass
#[test]
fn has_enum_needed() {
    has_needed::<OverwriteMethod>();
    has_needed::<InputError>();
    has_needed::<ProcessError>();
}
