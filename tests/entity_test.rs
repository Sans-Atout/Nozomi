use nozomi::enums::{error::Error, success::Success, erase_method::EraserEntity};

fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}

/// Test if all Entity in enum file has Send and Sync trait implemented
/// 
/// This test is sucessfull if all entity have the Sync and Send trait.
/// If the library compile, then this test must pass (has Send and Sync
///  traitis checked at compile time).
#[test]
fn has_enum_sync_send() {
    has_send_sync::<EraserEntity>();
    has_send_sync::<Error>();
    has_send_sync::<Success>();

}