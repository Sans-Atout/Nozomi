use std::fmt;

/// Nozomi Error management systems
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// If the folder at the given path didn't exist or is not a folder
    NotAFolderOrDidntExist,
    /// If the file at the given path didn't exist or is not a file   
    NotAFileOrDidntExist,
    /// Error when trying to retrieve the file name     
    ErrorGetFileName,
    /// Error when trying to rename the file    
    RenameError,
    /// Error when trying to remove file
    RemoveFileError,
    /// Error in buffer writing function           
    BufferWritingError,
    /// Error in the fourth overwriting of DOD 522022 MECE      
    Dod522022MECEErrorFourth,
    /// Error in the fifth overwriting of DOD 522022 MECE  
    Dod522022MECEErrorFifth,
    /// Error in the sixth overwriting of DOD 522022 MECE
    Dod522022MECEErrorSixth,
    /// Error in the seventh overwriting of DOD 522022 MECE
    Dod522022MECEErrorSeventh,
    /// Error in the first overwriting of DOD 522022 ME
    Dod522022MEErrorFirst,
    /// Error in the second overwriting of DOD 522022 ME
    Dod522022MEErrorSecond,
    /// Error in the third overwriting of DOD 522022 ME
    Dod522022MEErrorThird,
    /// Error in the first overwriting of AFSSI 5020
    Afssi5020ErrorFirst,
    /// Error in the second overwriting of AFSSI 5020
    Afssi5020ErrorSecond,
    /// Error in the third overwriting of AFSSI 5020
    Afssi5020ErrorThird,
    /// Error in the first overwriting of RCMP TSSIT OPS II       
    RcmpTssitOpsIIErrorFirst,
    /// Error in the second overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorSecond,
    /// Error in the third overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorThird,
    /// Error in the fourth overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorFourth,
    /// Error in the fifth overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorFifth,
    /// Error in the sixth overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorSixth,
    /// Error in the seventh overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorSeventh,
    /// Error in the first overwriting with HMGI S5 method
    HmgiS5ErrorFirst,
    /// Error in the first overwriting with HMGI S5 method     
    HmgiS5ErrorSecond,
    /// Error in the overwriting with PseudoRandom method      
    PseudoRandomError,
    /// Error in file opening    
    FileOpeningError,
    /// One of the verification Fail        
    VerificationFailed,
    /// Error in buffer reading  
    BufferReadingError,
    /// Remove folder error       
    RemoveDirError,
    /// Gutman random patern overwriting problem Error       
    GutmannRandomPaternError,
    /// Gutman specific patern overwriting problem Error   
    GutmannSpecificPaternError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotAFolderOrDidntExist => write!(
                f,
                "The folder dind't exist or the path given is not a folder"
            ),
            Error::NotAFileOrDidntExist => {
                write!(f, "The file dind't exist or the path given is not a file")
            }
            Error::ErrorGetFileName => {
                write!(f, "There was an error in retrieving the name of a file")
            }
            Error::RenameError => write!(f, "There was an error when trying to rename a file"),
            Error::RemoveFileError => write!(f, "There was an error when trying to remove a file"),
            Error::BufferWritingError => {
                write!(f, "There was an error when trying to overwriting a file")
            }
            Error::Dod522022MECEErrorFourth => {
                write!(f, "Error in the fourth overwritting of DOD 522022 MECE")
            }
            Error::Dod522022MECEErrorFifth => {
                write!(f, "Error in the fifth overwritting of DOD 522022 MECE")
            }
            Error::Dod522022MECEErrorSixth => {
                write!(f, "Error in the sixth overwritting of DOD 522022 MECE")
            }
            Error::Dod522022MECEErrorSeventh => {
                write!(f, "Error in the seventh overwritting of DOD 522022 MECE")
            }
            Error::Dod522022MEErrorFirst => {
                write!(f, "Error in the first overwritting of DOD 522022 ME")
            }
            Error::Dod522022MEErrorSecond => {
                write!(f, "Error in the second overwritting of DOD 522022 ME")
            }
            Error::Dod522022MEErrorThird => {
                write!(f, "Error in the third overwritting of DOD 522022 ME")
            }
            Error::Afssi5020ErrorFirst => {
                write!(f, "Error in the first overwritting of AFSSI 5020")
            }
            Error::Afssi5020ErrorSecond => {
                write!(f, "Error in the second overwritting of AFSSI 5020")
            }
            Error::Afssi5020ErrorThird => {
                write!(f, "Error in the third overwritting of AFSSI 5020")
            }
            Error::RcmpTssitOpsIIErrorFirst => {
                write!(f, "Error in the first overwritting of RCMP TSSIT OPS II")
            }
            Error::RcmpTssitOpsIIErrorSecond => {
                write!(f, "Error in the second overwritting of RCMP TSSIT OPS II")
            }
            Error::RcmpTssitOpsIIErrorThird => {
                write!(f, "Error in the third overwritting of  RCMP TSSIT OPS II")
            }
            Error::RcmpTssitOpsIIErrorFourth => {
                write!(f, "Error in the fourth overwritting of RCMP TSSIT OPS II")
            }
            Error::RcmpTssitOpsIIErrorFifth => {
                write!(f, "Error in the fifth overwritting of RCMP TSSIT OPS II")
            }
            Error::RcmpTssitOpsIIErrorSixth => {
                write!(f, "Error in the sixth overwritting of RCMP TSSIT OPS II")
            }
            Error::RcmpTssitOpsIIErrorSeventh => {
                write!(f, "Error in the seventh overwritting of RCMP TSSIT OPS II")
            }
            Error::HmgiS5ErrorFirst => write!(f, "Error in the first overwritting of HMGI S5"),
            Error::HmgiS5ErrorSecond => write!(f, "Error in the second overwritting of HMGI S5"),
            Error::PseudoRandomError => write!(f, "Error in the third overwritting of HMGI S5"),
            Error::FileOpeningError => write!(f, "Error when trying to opening a file"),
            Error::VerificationFailed => write!(f, "Some of verification failed"),
            Error::BufferReadingError => write!(f, "Error in buffer reading"),
            Error::RemoveDirError => write!(f, "Error when removing a folder"),
            Error::GutmannRandomPaternError => {
                write!(f, "Error in the random patern overwritting of Gutmann")
            }
            Error::GutmannSpecificPaternError => {
                write!(f, "Error in the specific patern overwritting of Gutmann")
            }
        }
    }
}
