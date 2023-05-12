use nozomi::enums::{error::Error, success::Success, erase_method::EraserEntity};

fn has_send_sync<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn has_enum_sync_send() {
    has_send_sync::<EraserEntity>();
    has_send_sync::<Error>();
    has_send_sync::<Success>();

}