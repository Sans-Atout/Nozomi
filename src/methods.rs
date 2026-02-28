// -- Region : Extern library import
use std::path::Path;

// -- Region : feature import
#[cfg(not(feature = "error-stack"))]
use crate::Result;
use crate::api::delete::request::NoopSink;
#[cfg(feature = "log")]
use log::{error, info, warn};

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::{Context, Report, ResultExt};

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
        run(self, path_to_delete,&mut sink)
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
