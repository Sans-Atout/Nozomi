use nozomi::{OverwriteMethod, error::{InputError, ProcessError}};



fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}
fn has_needed<T: Copy + Clone>() {}

/// Test if all Entity in enum file has Send and Sync trait implemented
/// 
/// This test is sucessfull if all entity have the Sync and Send trait.
/// If the library compile, then this test must pass (has Send and Sync
///  traitis checked at compile time).
#[test]
fn has_enum_sync_send() {
    has_send_sync::<OverwriteMethod>();
    has_send_sync::<InputError>();
    has_send_sync::<ProcessError>();

}

#[test]
fn has_enum_needed(){
    has_needed::<OverwriteMethod>();
    has_needed::<InputError>();
    has_needed::<ProcessError>();
}