//! These tests ensure that each feature compiles independently.
//! They do not test behavior yet — only conditional compilation integrity.

/// Always compiled: ensures base crate compiles without features.
#[test]
fn base_build_compiles() {}

//
// VERIFY FEATURE
//

#[cfg(feature = "verify")]
mod verify_enabled {

    #[test]
    fn verify_feature_compiles() {}
}

//
// DRY-RUN FEATURE
//

#[cfg(feature = "dry-run")]
mod dry_run_enabled {
    #[test]
    fn dry_run_feature_compiles() {}
}

//
// ANALYZE FEATURE
//

#[cfg(feature = "analyze")]
mod analyze_enabled {
    #[test]
    fn analyze_feature_compiles() {}
}

//
// ERROR-STACK FEATURE
//

#[cfg(feature = "error-stack")]
mod error_stack_enabled {

    #[test]
    fn error_stack_feature_compiles() {}
}

//
// LOG FEATURE
//

#[cfg(feature = "log")]
mod log_enabled {

    #[test]
    fn log_feature_compiles() {}
}

//
// SECURE_LOG FEATURE
//

#[cfg(feature = "secure_log")]
mod secure_log_enabled {

    #[test]
    fn secure_log_feature_compiles() {}
}
