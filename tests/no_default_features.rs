use nozomi::*;
#[cfg(all(
    not(feature = "verify"),
    not(feature = "dry-run"),
    not(feature = "analyze"),
    not(feature = "error-stack"),
    not(feature = "log"),
    not(feature = "secure_log")
))]
#[test]
fn no_default_features_enabled() {
    assert!(true);
}
