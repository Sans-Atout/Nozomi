// Librairie use in this librairie
use rand;
use std::{fs,fmt};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

/// Nozomi Eraser method enume
/// Based on Eraser for Windows main method
#[derive(Debug, Clone, Copy)]
pub enum EraserEntity {
    /// DOD 522022 MECE erasing method <https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php>X
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
    PseudoRandom 
}

/// Erase folder method
/// 
/// Function that allows you to delete, recursively or not, a folder according to a given secure deletion method 
/// 
/// * `_path`        - The path of the folder you wish to delete
/// * `erase_method` - The secure file deletion method (EraserEntity enumeration)
/// * `is_recursive` - Does the deletion have to be recursive or not ?
pub fn erase_folder(_path : &str, erase_method : EraserEntity, is_recursive: bool) -> Result<Success, Error>{
    let _p = Path::new(_path);
    if ! (_p.exists() && _p.is_dir()){
        return Err(Error::NotAFolderOrDidntExist)
    }

    let paths = fs::read_dir(_path).unwrap();
    for path in paths {
        let temp_path = path.unwrap().path();
        let _file_name = match temp_path.to_str(){
            Some(file) => file,
            None => return Err(Error::ErrorGetFileName)
        };
        let is_folder = Path::new(_file_name).is_dir();
        if is_folder {
            if is_recursive{
                match erase_folder(_file_name,erase_method, is_recursive){
                    Ok(_) => true,
                    Err(error) => return Err(error)
                };
            }
        }
        else{
            match erase_file(_file_name,erase_method){
                Ok(_) => true,
                Err(error) => return Err(error)
            };
        }

    }
    match fs::remove_dir_all(_path){
        Ok(_) => Ok(Success::EraseFolderSuccess),
        Err(_) => Err(Error::RemoveDirError)
    }
}

/// Erase one file method
/// 
/// Function that allows you to delete, recursively or not, a file according to a given secure deletion method 
/// 
/// * `_path`        - The path of the file you wish to delete
/// * `erase_method` - The secure file deletion method (EraserEntity enumeration)
pub fn erase_file(_path : &str, erase_method : EraserEntity) -> Result<Success, Error>{
    let _p = Path::new(_path);
    if ! (_p.exists() && _p.is_file()){
        return Err(Error::NotAFileOrDidntExist)
    }
    match erase_method{
        EraserEntity::Gutmann => {
            match gutmann_overwrite_file(_path) {
                Ok(_) => (),
                Err(error) => return Err(error)
            };
        },
        EraserEntity::HmgiS5 => {
            match hmgi_s5_overwrite_file(_path){
                Ok(_) => (),
                Err(error) => return Err(error)
            };
        },
        EraserEntity::RcmpTssitOpsII =>{
            match rcmp_tssit_ops_ii_overwrite_file(_path){
                Ok(_) => (),
                Err(error) => return Err(error)
            };
        },
        EraserEntity::PseudoRandom =>{
            match file_overwriting_random(_path){
                Ok(_) => (),
                Err(error) => return Err(error)
            }
        },
        EraserEntity::Afssi5020 => {
            match afssi_5020_overwrite_file(_path){
                Ok(_) => (),
                Err(error) => return Err(error)
            }
        },
        EraserEntity::Dod522022MECE => {
            match dod_522022_mece_overwrite_file(_path){
                Ok(_) => (),
                Err(error) => return Err(error)
            }
        },
        EraserEntity::Dod522022ME => {
            match dod_522022_me_overwrite_file(_path){
                Ok(_) => (),
                Err(error) => return Err(error)
            }
        },
    }
    
    match delete_file(String::from(_path)) {
        Ok(_) => true,
        Err(error) => return Err(error)
    };
    Ok(Success::EraseFileSuccess)
}

fn file_overwriting(_path : &str, _char : [u8; 3])-> Result<Success, Error> {
    // Declare important variable for this 
    let mut _file = match File::options().read(true).open(_path){
        Ok(file) => file, 
        Err(_) => return Err(Error::FileOpeningError)
    };
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();

    let _d = match reader.read_to_end(&mut buffer){
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError)
    };
    
    for _id in 0..buffer.len() { buffer_modified.push(_char[_id % 3]);}

    _file = match File::options().write(true).open(_path){
        Ok(file) => file,  
        Err(_) => return Err(Error::FileOpeningError)
    };
    match _file.write_all(buffer_modified.as_slice()){
        Ok(_) => true,
        Err(_) => return Err(Error::BufferWritingError)
    };

    Ok(Success::FileOverWriting)
}

fn file_overwriting_random(_path : &str)-> Result<Success, Error> {
    // Declare important variable for this 
    let mut _file = match File::options().read(true).open(_path){
        Ok(file) => file, 
        Err(_) => return Err(Error::FileOpeningError)
    };
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();
    
    let _d = match reader.read_to_end(&mut buffer){
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError)
    };

    for _id in 0..buffer.len() {
        let mut _rand:u8 = rand::random(); 
        buffer_modified.push(_rand);
    }

     _file = match File::options().write(true).open(_path){
        Ok(file) => file, 
        Err(_) => return Err(Error::FileOpeningError)
    };
    match _file.write_all(buffer_modified.as_slice()){
        Ok(_) => true,
        Err(_) => return Err(Error::BufferWritingError)
    };

    Ok(Success::FileOverWriting)
}

fn file_overwriting_hexa(_path : &str, _char : u8)-> Result<Success, Error> {
    // Declare important variable for this 
    let mut _file = match File::options().read(true).open(_path){
        Ok(file) => file, 
        Err(_) => return Err(Error::FileOpeningError)
    };
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();
    
    let _d = match reader.read_to_end(&mut buffer){
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError)
    };
    
    for _id in 0..buffer.len() { buffer_modified.push(_char);}
    
    _file = match File::options().write(true).open(_path){
        Ok(file) => file, 
        Err(_) => return Err(Error::FileOpeningError)
    };
    match _file.write_all(buffer_modified.as_slice()){
        Ok(_) => true,
        Err(_) => return Err(Error::BufferWritingError)
    };

    Ok(Success::FileOverWriting)
}

fn gutmann_overwrite_file(_path : &str)-> Result<Success, Error> {
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    match file_overwriting(_path,[0x55 as u8,0x55 as u8,0x55 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xAA as u8,0xAA as u8,0xAA as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x92 as u8,0x49 as u8,0x24 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x00 as u8,0x00 as u8,0x00 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x11 as u8,0x11 as u8,0x11 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x22 as u8,0x22 as u8,0x22 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x33 as u8,0x33 as u8,0x33 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x44 as u8,0x44 as u8,0x44 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x55 as u8,0x55 as u8,0x55 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x66 as u8,0x66 as u8,0x66 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x77 as u8,0x77 as u8,0x77 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x88 as u8,0x88 as u8,0x88 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x99 as u8,0x99 as u8,0x99 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xAA as u8,0xAA as u8,0xAA as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xBB as u8,0xBB as u8,0xBB as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xCC as u8,0xCC as u8,0xCC as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xDD as u8,0xDD as u8,0xDD as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xEE as u8,0xEE as u8,0xEE as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xFF as u8,0xFF as u8,0xFF as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x92 as u8,0x49 as u8,0x24 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x49 as u8,0x24 as u8,0x92 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0x6D as u8,0xB6 as u8,0xDB as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xB6 as u8,0xDB as u8,0x6D as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting(_path,[0xDB as u8,0x6D as u8,0xB6 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannSpecificPaternError)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::GutmannRandomPaternError)
    };
    Ok(Success::GutmannSuccess)
}

fn hmgi_s5_overwrite_file(_path : &str)-> Result<Success, Error >{
    match file_overwriting(_path,[0x00 as u8,0x00 as u8,0x00 as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::HmgiS5ErrorFirst)
    };
    match file_overwriting(_path,[0xFF as u8,0xFF as u8,0xFF as u8]){
        Ok(_) => true,
        Err(_) => return Err(Error::HmgiS5ErrorSecond)
    };
    let mut _file = match File::options().read(true).open(_path){
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError)
    };

    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let _d = match reader.read_to_end(&mut buffer){
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError)
    };
    for _char in buffer { 
        if ! _char as u8 == 0xFF as u8{
            return Err(Error::VerificationFailed)
        }
    }
    return Ok(Success::HmgiS5Sucess)
}

fn rcmp_tssit_ops_ii_overwrite_file(_path : &str) -> Result<Success, Error >{
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorFirst)
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => (),
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorSecond)
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorThird)
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorFourth)
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorFifth)
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorSixth)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::RcmpTssitOpsIIErrorSeventh)
    };
    Ok(Success::RcmpTssitOpsIISucess)
}

fn afssi_5020_overwrite_file(_path : &str) -> Result<Success, Error >{
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::Afssi5020ErrorFirst)
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::Afssi5020ErrorSecond)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::Afssi5020ErrorThird)
    };
    Ok(Success::Afssi5020Success)
}

fn dod_522022_mece_overwrite_file(_path : &str) -> Result<Success, Error >{
    match dod_522022_me_overwrite_file(_path){
        Ok(_) => true,
        Err(error) => return Err(error)
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorFourth)
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorFifth)
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorSixth)
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MECEErrorFourth)
    };
    Ok(Success::Dod522022MECESuccess)
}

fn dod_522022_me_overwrite_file(_path : &str) -> Result<Success, Error >{
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MEErrorFirst)
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MEErrorSecond)
    };
    match file_overwriting(_path,get_random_patern()){
        Ok(_) => true,
        Err(_) => return Err(Error::Dod522022MEErrorThird)
    };
    Ok(Success::Dod522022MESucess)
}

fn get_random_patern()->[u8; 3]{
    let x1:u8 = rand::random();
    let x2:u8 = rand::random(); 
    let x3:u8 = rand::random(); 
    return [x1,x2,x3]
}

fn delete_file(_path : String)-> Result<Success, Error>{
    let mut new_path = _path.clone();
    let size = match get_file_name_size(new_path.clone()){
        Ok(s) => s,
        Err(error) => return Err(error)
    };

    for  s in 0..size+1 {
        new_path = match rename_file(new_path, size -s ){
            Ok(file) => file,
            Err(_) => return Err(Error::RenameError)
        };

    }
    match fs::remove_file(new_path){
         Ok(_) => true,
         Err(_) => return Err(Error::RemoveFileError)
    };
    
    Ok(Success::DeleteFileSuccess)
}

fn generate_zero_string(size : u32) -> String{
    let mut _string = String::from("");
    let mut s_size = 0;
    while s_size <= size {
        _string += "0";
        s_size += 1;
    }
    return _string
}

fn rename_file(_path : String, size : u32) -> Result<String, Error>{
    let _p = Path::new(&_path);
    if ! (_p.exists() && _p.is_file()){
        return Err(Error::NotAFileOrDidntExist)
    }
    let _file_name = match _p.file_name(){
        Some(file) => file,
        None => return Err(Error::ErrorGetFileName)
    };
    let dir = _p.parent().unwrap().to_str().unwrap();
    let new_file_name = String::from([dir, &generate_zero_string(size).as_str()].join("/"));
    match fs::rename(&_path,&new_file_name){
        Ok(_) => true,
        Err(_) => return Err(Error::RenameError)
    };
    Ok(new_file_name)
}

fn get_file_name_size(new_path : String) -> Result<u32, Error>{
    let _p = Path::new(&new_path);
    if ! (_p.exists() && _p.is_file()){
        return Err(Error::NotAFileOrDidntExist)
    }
    let file_name = match _p.file_name(){
        Some(file) => file,
        None => return Err(Error::ErrorGetFileName)
    };
    Ok(file_name.len() as u32)
}

/// Nozomi Error management systems
#[derive(Debug, Clone, Copy)]
pub enum Error{
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

/// Nozomi Sucesss management systems
#[derive(Debug, Clone, Copy)]
pub enum Success{ 
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

impl fmt::Display for Success{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Success::Dod522022MECESuccess => write!(f,"Your erase file with DOD 522022 MECE method successfuly"),
            Success::Dod522022MESucess => write!(f,"Your erase file with DOD 522022 ME method successfuly"),
            Success::Afssi5020Success => write!(f,"Your erase file with AFSSI 5020 method successfuly"),
            Success::RcmpTssitOpsIISucess => write!(f,"Your erase file with RCMP TSSIT OPS II method successfuly"),
            Success::HmgiS5Sucess => write!(f,"You erase file with HMGI S5 method successfuly"),
            Success::GutmannSuccess => write!(f, "You erase file with Gutmann method successfuly"),
            Success::PseudoRandomSuccess => write!(f, "You erase file with Pseudo Random method successfuly"),
            Success::EraseFolderSuccess => write!(f, "You successfuly erase a folder"),
            Success::EraseFileSuccess => write!(f, "You successfuly erase a file"),
            Success::DeleteFileSuccess => write!(f,"You delete a file successfuly"),
            Success::FileOverWriting => write!(f,"The file was successfuly overwritten")
        }

    }
}

impl fmt::Display for Error{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotAFolderOrDidntExist => write!(f,"The folder dind't exist or the path given is not a folder"), 
            Error::NotAFileOrDidntExist => write!(f,"The file dind't exist or the path given is not a file"), 
            Error::ErrorGetFileName => write!(f,"There was an error in retrieving the name of a file"),          
            Error::RenameError => write!(f,"There was an error when trying to rename a file"), 
            Error::RemoveFileError => write!(f,"There was an error when trying to remove a file"), 
            Error::BufferWritingError => write!(f,"There was an error when trying to overwriting a file"),  
            Error::Dod522022MECEErrorFourth => write!(f,"Error in the fourth overwritting of DOD 522022 MECE"),   
            Error::Dod522022MECEErrorFifth => write!(f,"Error in the fifth overwritting of DOD 522022 MECE"),   
            Error::Dod522022MECEErrorSixth => write!(f,"Error in the sixth overwritting of DOD 522022 MECE"),       
            Error::Dod522022MECEErrorSeventh => write!(f,"Error in the seventh overwritting of DOD 522022 MECE"),   
            Error::Dod522022MEErrorFirst => write!(f,"Error in the first overwritting of DOD 522022 ME"),      
            Error::Dod522022MEErrorSecond => write!(f,"Error in the second overwritting of DOD 522022 ME"),     
            Error::Dod522022MEErrorThird => write!(f,"Error in the third overwritting of DOD 522022 ME"),       
            Error::Afssi5020ErrorFirst => write!(f,"Error in the first overwritting of AFSSI 5020"),        
            Error::Afssi5020ErrorSecond => write!(f,"Error in the second overwritting of AFSSI 5020"),       
            Error::Afssi5020ErrorThird => write!(f,"Error in the third overwritting of AFSSI 5020"),  
            Error::RcmpTssitOpsIIErrorFirst => write!(f,"Error in the first overwritting of RCMP TSSIT OPS II"),  
            Error::RcmpTssitOpsIIErrorSecond => write!(f,"Error in the second overwritting of RCMP TSSIT OPS II"),  
            Error::RcmpTssitOpsIIErrorThird => write!(f,"Error in the third overwritting of  RCMP TSSIT OPS II"),   
            Error::RcmpTssitOpsIIErrorFourth => write!(f,"Error in the fourth overwritting of RCMP TSSIT OPS II"),   
            Error::RcmpTssitOpsIIErrorFifth => write!(f,"Error in the fifth overwritting of RCMP TSSIT OPS II"),   
            Error::RcmpTssitOpsIIErrorSixth => write!(f,"Error in the sixth overwritting of RCMP TSSIT OPS II"),    
            Error::RcmpTssitOpsIIErrorSeventh => write!(f,"Error in the seventh overwritting of RCMP TSSIT OPS II"), 
            Error::HmgiS5ErrorFirst => write!(f,"Error in the first overwritting of HMGI S5"), 
            Error::HmgiS5ErrorSecond => write!(f,"Error in the second overwritting of HMGI S5"),  
            Error::PseudoRandomError => write!(f,"Error in the third overwritting of HMGI S5"),    
            Error::FileOpeningError => write!(f,"Error when trying to opening a file"),  
            Error::VerificationFailed => write!(f,"Some of verification failed"),     
            Error::BufferReadingError => write!(f,"Error in buffer reading"), 
            Error::RemoveDirError => write!(f,"Error when removing a folder"), 
            Error::GutmannRandomPaternError => write!(f,"Error in the random patern overwritting of Gutmann"), 
            Error::GutmannSpecificPaternError => write!(f,"Error in the specific patern overwritting of Gutmann"), 
        }

    }
}