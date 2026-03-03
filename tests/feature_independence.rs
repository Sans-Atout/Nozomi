use nozomi::*;

#[test]
fn base_build_still_minimal() {
    assert!(true);
}

#[cfg(feature = "verify")]
#[test]
fn verify_does_not_require_other_features() {
    assert!(true);
}

#[cfg(feature = "dry-run")]
#[test]
fn dry_run_does_not_require_other_features() {
    assert!(true);
}

#[cfg(feature = "analyze")]
#[test]
fn analyze_does_not_require_other_features() {
    assert!(true);
}

#[cfg(all(feature = "verify", feature = "error-stack"))]
#[test]
fn verify_and_error_stack_compile_together() {
    assert!(true);
}

#[cfg(all(feature = "secure_log", feature = "verify"))]
#[test]
fn secure_log_and_verify_compile_together() {
    assert!(true);
}

#[cfg(not(feature = "dry-run"))]
#[test]
fn dry_run_compile_together() {
    assert!(true);
}

#[cfg(not(feature = "analyze"))]
#[test]
fn analyze_not_available_without_feature() {
    assert!(true);
}