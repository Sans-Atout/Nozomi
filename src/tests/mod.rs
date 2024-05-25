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

const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent congue diam risus, quis hendrerit nunc commodo id. Duis volutpat vitae leo at malesuada. Phasellus libero nisl, auctor sit amet dapibus eget, egestas eget velit. Curabitur fermentum, libero vel dictum hendrerit, metus leo pellentesque tortor, et vehicula nisl felis elementum purus. Etiam eu ex in odio tincidunt maximus. Ut luctus blandit ligula et dignissim. Duis id imperdiet urna. Sed porttitor nulla vitae sollicitudin sagittis. Donec ut justo ut risus pretium dignissim. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec eu rhoncus erat, sed sagittis elit. Proin dui nisl, varius nec volutpat sed, lobortis nec tellus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.\n";
pub enum TestType {
    SmallFile, 
    MediumFile,
    LargeFile,
    WrittingError,
    Folder,
    OverwriteOnly
}