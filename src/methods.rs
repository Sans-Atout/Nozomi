// -- Region : Extern library import
use std::path::Path;

// -- Region : feature import
#[cfg(not(feature = "error-stack"))]
use crate::Result;
use crate::api::delete::request::NoopSink;

#[cfg(feature = "analyze")]
use crate::AnalysisReport;
#[cfg(feature = "error-stack")]
use crate::{Error, Result};
use crate::Result;
#[cfg(feature = "analyze")]
use crate::{PassInfo, PassKind};

use crate::engine::run;
// -- Region : Method logic

/// Nozomi Eraser method enumeration based on Eraser for Windows main method
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Method {
    /// DOD 522022 MECE erasing method <https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php>
    Dod522022MECE,
    /// DOD 522022 ME erasing method <https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php>
    Dod522022ME,
    /// AFSSI 5020 erasing method <https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020>
    Afssi5020,
    /// RCMP TSSIT OPS II erasing method <https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html>
    RcmpTssitOpsII,
    /// HMGI S5 erasing method <https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php>
    HmgiS5,
    /// Gutmann erasing method <https://en.wikipedia.org/wiki/Gutmann_method>
    Gutmann,
    /// Pseudo Random erasing method <https://www.lifewire.com/data-sanitization-methods-2626133#toc-random-data>
    #[default]
    PseudoRandom,
}

// -- Region : Implement logic for basic error handling.
#[cfg(not(feature = "error-stack"))]
impl Method {
    /// This function is used to delete a file or folder using a predefined method using basic error handling method.
    ///
    /// ## Argument :
    /// * `self` (&Method) : Nozomi Eraser method enumeration based on Eraser for Windows main method
    /// * `path` (&str) : path that you want to erase using the given overwrite method
    pub fn delete(&self, path: &str) -> Result<()> {
        let path_to_delete = Path::new(path);
        let mut sink = NoopSink;
        run(self, path_to_delete, &mut sink)
    }
}

// -- Region : Implement logic for error-stack's error handling.
#[cfg(feature = "error-stack")]
impl Method {
    /// This function is used to delete a file or folder using a predefined method using error-stack's error handling method.
    ///
    /// ## Argument :
    /// * `self` (&Method) : Nozomi Eraser method enumeration based on Eraser for Windows main method
    /// * `path` (&str) : path that you want to erase using the given overwrite method
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
