use std::fmt;

/// Nozomi Sucesss management systems
#[derive(Debug, Clone, Copy)]
pub enum Success {
    /// DOD 522022 MECE method success
    Dod522022MECESuccess,
    /// DOD 522022 ME method success
    Dod522022MESucess,
    /// AFSSI 5020 erasing method success
    Afssi5020Success,
    /// RCMP TSSIT OPS II method success
    RcmpTssitOpsIISucess,
    /// HMGI S5 method success
    HmgiS5Sucess,
    /// Gutmann method success   
    GutmannSuccess,
    /// Pseudo Random method success     
    PseudoRandomSuccess,
    /// The erase_folder method was completed without any errors.
    EraseFolderSuccess,
    /// The erase_file method was completed without any errors
    EraseFileSuccess,
    /// file_overwritting generic success return  
    FileOverWriting,
    /// The delete_file function ended successfully  
    DeleteFileSuccess,
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Success::Dod522022MECESuccess => {
                write!(f, "Your erase file with DOD 522022 MECE method successfuly")
            }
            Success::Dod522022MESucess => {
                write!(f, "Your erase file with DOD 522022 ME method successfuly")
            }
            Success::Afssi5020Success => {
                write!(f, "Your erase file with AFSSI 5020 method successfuly")
            }
            Success::RcmpTssitOpsIISucess => write!(
                f,
                "Your erase file with RCMP TSSIT OPS II method successfuly"
            ),
            Success::HmgiS5Sucess => write!(f, "You erase file with HMGI S5 method successfuly"),
            Success::GutmannSuccess => write!(f, "You erase file with Gutmann method successfuly"),
            Success::PseudoRandomSuccess => {
                write!(f, "You erase file with Pseudo Random method successfuly")
            }
            Success::EraseFolderSuccess => write!(f, "You successfuly erase a folder"),
            Success::EraseFileSuccess => write!(f, "You successfuly erase a file"),
            Success::DeleteFileSuccess => write!(f, "You delete a file successfuly"),
            Success::FileOverWriting => write!(f, "The file was successfuly overwritten"),
        }
    }
}
