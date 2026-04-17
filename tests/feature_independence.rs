#[test]
fn base_build_still_minimal() {}

#[cfg(feature = "verify")]
#[test]
fn verify_does_not_require_other_features() {}

#[cfg(feature = "dry-run")]
#[test]
fn dry_run_does_not_require_other_features() {}

#[cfg(feature = "analyze")]
#[test]
fn analyze_does_not_require_other_features() {}

#[cfg(all(feature = "verify", feature = "error-stack"))]
#[test]
fn verify_and_error_stack_compile_together() {}

#[cfg(all(feature = "secure_log", feature = "verify"))]
#[test]
fn secure_log_and_verify_compile_together() {}

#[cfg(not(feature = "dry-run"))]
#[test]
fn dry_run_compile_together() {}

#[cfg(not(feature = "analyze"))]
#[test]
fn analyze_not_available_without_feature() {}
