// -- Region : Extern library import
use std::path::Path;

// -- Region : feature import
#[cfg(not(feature = "error-stack"))]
use crate::Result;
use crate::api::delete::request::NoopSink;

#[cfg(feature = "analyze")]
use crate::AnalysisReport;
#[cfg(feature = "error-stack")]
#[allow(deprecated)]
use crate::Result;
#[cfg(feature = "analyze")]
use crate::{PassInfo, PassKind};

use crate::engine::run;
// -- Region : Method logic

/// Identifies the overwrite algorithm to use when securely deleting a file.
///
/// Each variant maps to an industry-standard data sanitisation method. The
/// variants are ordered roughly by the number of overwrite passes they perform,
/// from fastest to most thorough.
///
/// `PseudoRandom` is the default and is suitable for most use cases. Use
/// `Gutmann` only when you require the maximum theoretical guarantee for
/// magnetic media.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Method {
    /// [DoD 5220.22-M (ECE)](https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php) — 7 passes.
    Dod522022MECE,
    /// [DoD 5220.22-M (ME)](https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php) — 3 passes.
    Dod522022ME,
    /// [AFSSI 5020](https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020) — 3 passes.
    Afssi5020,
    /// [RCMP TSSIT OPS-II](https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html) — 7 passes.
    RcmpTssitOpsII,
    /// [HMGI S5](https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php) — 2 passes.
    HmgiS5,
    /// [Gutmann](https://en.wikipedia.org/wiki/Gutmann_method) — 35 passes.
    Gutmann,
    /// Single-pass pseudo-random overwrite. Fast and suitable for modern storage. Default method.
    #[default]
    PseudoRandom,
}

// -- Region : Implement logic for basic error handling.
#[cfg(not(feature = "error-stack"))]
impl Method {
    /// Securely overwrites and deletes the file or directory at `path` using
    /// this method's algorithm.
    ///
    /// This is the high-level convenience entry point for the deprecated
    /// single-argument API. Prefer [`DeleteRequest::builder`](crate::DeleteRequest::builder)
    /// for new code as it provides event observation and dry-run support.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the path does not exist, cannot be
    /// overwritten, or cannot be removed.
    pub fn delete(&self, path: &str) -> Result<()> {
        let path_to_delete = Path::new(path);
        let mut sink = NoopSink;
        run(self, path_to_delete, &mut sink)
    }
}

// -- Region : Implement logic for error-stack's error handling.
#[cfg(feature = "error-stack")]
#[allow(deprecated)]
impl Method {
    /// Securely overwrites and deletes the file or directory at `path` using
    /// this method's algorithm.
    ///
    /// This is the high-level convenience entry point for the deprecated
    /// single-argument API. Prefer [`DeleteRequest::builder`](crate::DeleteRequest::builder)
    /// for new code as it provides event observation and dry-run support.
    ///
    /// # Errors
    ///
    /// Returns an [`error_stack::Report`] wrapping [`Error`](crate::Error) if
    /// the path does not exist, cannot be overwritten, or cannot be removed.
    pub fn delete(&self, path: &str) -> Result<()> {
        let path_to_delete = Path::new(path);
        let mut sink = NoopSink;
        run(self, path_to_delete, &mut sink)
    }
}

// -- Region : Implement display trait for Method enum.
impl core::fmt::Display for Method {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Method::Dod522022MECE => write!(fmt, "DOD 522022 MECE"),
            Method::Dod522022ME => write!(fmt, "DOD 522022 ME"),
            Method::Afssi5020 => write!(fmt, "AFSSI 5020"),
            Method::RcmpTssitOpsII => write!(fmt, "RCMP TSSIT OPS II"),
            Method::HmgiS5 => write!(fmt, "HMGI S5"),
            Method::Gutmann => write!(fmt, "Gutmann"),
            Method::PseudoRandom => write!(fmt, "Pseudo Random"),
        }
    }
}

#[cfg(feature = "analyze")]
impl Method {
    /// Returns a static description of the overwrite schedule applied by this
    /// method, without executing any I/O.
    ///
    /// The resulting [`AnalysisReport`] lists every pass in order with its
    /// [`PassKind`](crate::PassKind), and can be used to display or audit the
    /// plan before running a deletion.
    ///
    /// Only available when the `analyze` feature is enabled.
    pub fn analyze(&self) -> AnalysisReport {
        match &self {
            Method::Dod522022MECE => AnalysisReport {
                pass_count: 6,
                passes: vec![
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                    PassInfo {
                        kind: PassKind::One,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                    PassInfo {
                        kind: PassKind::One,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                ],
            },
            Method::Dod522022ME | Method::Afssi5020 => AnalysisReport {
                pass_count: 3,
                passes: vec![
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                    PassInfo {
                        kind: PassKind::One,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                ],
            },
            Method::RcmpTssitOpsII => AnalysisReport {
                pass_count: 6,
                passes: vec![
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                    PassInfo {
                        kind: PassKind::One,
                    },
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                    PassInfo {
                        kind: PassKind::One,
                    },
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                    PassInfo {
                        kind: PassKind::One,
                    },
                ],
            },
            Method::HmgiS5 => AnalysisReport {
                pass_count: 2,
                passes: vec![
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                    PassInfo {
                        kind: PassKind::Zero,
                    },
                ],
            },
            Method::Gutmann => AnalysisReport {
                pass_count: 35,
                passes: vec![
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x55, 0x55, 0x55]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xAA, 0xAA, 0xAA]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x92, 0x49, 0x24]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x49, 0x24, 0x92]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x24, 0x92, 0x49]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x00, 0x00, 0x00]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x11, 0x11, 0x11]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x22, 0x22, 0x22]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x33, 0x33, 0x33]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x44, 0x44, 0x44]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x55, 0x55, 0x55]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x66, 0x66, 0x66]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x77, 0x77, 0x77]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x88, 0x88, 0x88]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x99, 0x99, 0x99]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xAA, 0xAA, 0xAA]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xBB, 0xBB, 0xBB]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xCC, 0xCC, 0xCC]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xDD, 0xDD, 0xDD]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xEE, 0xEE, 0xEE]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xFF, 0xFF, 0xFF]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x92, 0x49, 0x24]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x49, 0x24, 0x92]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x24, 0x92, 0x49]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0x6D, 0xB6, 0xDB]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xB6, 0xDB, 0x6D]),
                    },
                    PassInfo {
                        kind: PassKind::ThreeBytePattern([0xDB, 0x6D, 0xB6]),
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                    PassInfo {
                        kind: PassKind::Random,
                    },
                ],
            },
            Method::PseudoRandom => AnalysisReport {
                pass_count: 1,
                passes: vec![PassInfo {
                    kind: PassKind::Random,
                }],
            },
        }
    }
}

#[cfg(feature = "analyze")]
#[cfg(test)]
mod analyse_tests {
    use super::Method;
    use crate::analyze::{AnalysisReport, PassInfo, PassKind};
    use pretty_assertions::assert_eq;

    #[test]
    fn gutmann() {
        let expected = AnalysisReport {
            pass_count: 35,
            passes: vec![
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x55, 0x55, 0x55]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xAA, 0xAA, 0xAA]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x92, 0x49, 0x24]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x49, 0x24, 0x92]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x24, 0x92, 0x49]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x00, 0x00, 0x00]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x11, 0x11, 0x11]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x22, 0x22, 0x22]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x33, 0x33, 0x33]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x44, 0x44, 0x44]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x55, 0x55, 0x55]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x66, 0x66, 0x66]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x77, 0x77, 0x77]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x88, 0x88, 0x88]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x99, 0x99, 0x99]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xAA, 0xAA, 0xAA]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xBB, 0xBB, 0xBB]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xCC, 0xCC, 0xCC]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xDD, 0xDD, 0xDD]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xEE, 0xEE, 0xEE]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xFF, 0xFF, 0xFF]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x92, 0x49, 0x24]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x49, 0x24, 0x92]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x24, 0x92, 0x49]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0x6D, 0xB6, 0xDB]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xB6, 0xDB, 0x6D]),
                },
                PassInfo {
                    kind: PassKind::ThreeBytePattern([0xDB, 0x6D, 0xB6]),
                },
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
            ],
        };
        assert_eq!(expected, Method::Gutmann.analyze());
    }

    #[test]
    fn pseudo_random() {
        let expected = AnalysisReport {
            pass_count: 1,
            passes: vec![PassInfo {
                kind: PassKind::Random,
            }],
        };
        assert_eq!(expected, Method::PseudoRandom.analyze());
    }

    #[test]
    fn hmgi_s5() {
        let expected = AnalysisReport {
            pass_count: 2,
            passes: vec![
                PassInfo {
                    kind: PassKind::Zero,
                },
                PassInfo {
                    kind: PassKind::Zero,
                },
            ],
        };
        assert_eq!(expected, Method::HmgiS5.analyze());
    }

    #[test]
    fn rcmp_tssit_ops_ii() {
        let expected = AnalysisReport {
            pass_count: 6,
            passes: vec![
                PassInfo {
                    kind: PassKind::Zero,
                },
                PassInfo {
                    kind: PassKind::One,
                },
                PassInfo {
                    kind: PassKind::Zero,
                },
                PassInfo {
                    kind: PassKind::One,
                },
                PassInfo {
                    kind: PassKind::Zero,
                },
                PassInfo {
                    kind: PassKind::One,
                },
            ],
        };
        assert_eq!(expected, Method::RcmpTssitOpsII.analyze())
    }

    #[test]
    fn dod_522022_me() {
        let expected = AnalysisReport {
            pass_count: 3,
            passes: vec![
                PassInfo {
                    kind: PassKind::Zero,
                },
                PassInfo {
                    kind: PassKind::One,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
            ],
        };
        assert_eq!(expected, Method::Dod522022ME.analyze());
    }

    #[test]
    fn dod_522022_mece() {
        let expected = AnalysisReport {
            pass_count: 6,
            passes: vec![
                PassInfo {
                    kind: PassKind::Zero,
                },
                PassInfo {
                    kind: PassKind::One,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
                PassInfo {
                    kind: PassKind::Zero,
                },
                PassInfo {
                    kind: PassKind::One,
                },
                PassInfo {
                    kind: PassKind::Random,
                },
            ],
        };
        assert_eq!(expected, Method::Dod522022MECE.analyze());
    }
}
