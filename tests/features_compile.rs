//! These tests ensure that each feature compiles independently.
//! They do not test behavior yet — only conditional compilation integrity.

use nozomi::*;

/// Always compiled: ensures base crate compiles without features.
#[test]
fn base_build_compiles() {
    // If this test runs, the default feature set compiled successfully.
    assert!(true);
}

//
// VERIFY FEATURE
//

#[cfg(feature = "verify")]
mod verify_enabled {
    use super::*;

    #[test]
    fn verify_feature_compiles() {
        // Placeholder until builder method is implemented.
        // Ensures the crate compiles with `--features verify`.
        assert!(true);
    }
}

//
// DRY-RUN FEATURE
//

#[cfg(feature = "dry-run")]
mod dry_run_enabled {
    use super::*;

    #[test]
    fn dry_run_feature_compiles() {
        assert!(true);
    }
}

//
// ANALYZE FEATURE
//

#[cfg(feature = "analyze")]
mod analyze_enabled {
    use super::*;

    #[test]
    fn analyze_feature_compiles() {
        assert!(true);
    }
}

//
// ERROR-STACK FEATURE
//

#[cfg(feature = "error-stack")]
mod error_stack_enabled {
    use super::*;

    #[test]
    fn error_stack_feature_compiles() {
        assert!(true);
    }
}

//
// LOG FEATURE
//

#[cfg(feature = "log")]
mod log_enabled {
    use super::*;

    #[test]
    fn log_feature_compiles() {
        assert!(true);
    }
}

//
// SECURE_LOG FEATURE
//

#[cfg(feature = "secure_log")]
mod secure_log_enabled {
    use super::*;

    #[test]
    fn secure_log_feature_compiles() {
        assert!(true);
    }
}
