// Librairie use in this librairie
use rand;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

/// Erase entity
#[derive(Debug, Clone, Copy)]
pub enum EraserEntity {
    Dod522022MECE,
    Dod522022ME,
    Afssi5020,
    RcmpTssitOpsII,
    HmgiS5,
    Gutmann,
    PseudoRandom
}

pub fn erase_folder(_path : &str, erase_method : EraserEntity, is_recursive: bool) -> Result<bool, Error>{
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
        if is_recursive{
            let is_folder = Path::new(_file_name).is_dir();
            if is_folder {
                match erase_folder(_file_name,erase_method, is_recursive){
                    Ok(_) => true,
                    Err(error) => return Err(error)
                };
            }
        }
        match erase_file(_file_name,erase_method){
            Ok(_) => true,
            Err(error) => return Err(error)
        };

    }
    match fs::remove_dir_all(_path){
        Ok(_) => Ok(true),
        Err(_) => Err(Error::RemoveDirError)
    }
}

/// Erase one file wirh a giver erase method from EraserEntity
pub fn erase_file(_path : &str, erase_method : EraserEntity) -> Result<bool, Error>{
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
        Ok(_) => (),
        Err(_) => return Ok(false)
    };
    Ok(true)
    // one_file_pass()
}

fn file_overwriting(_path : &str, _char : [u8; 3])-> Result<bool, Error> {
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

    Ok(true)
}

fn file_overwriting_random(_path : &str)-> Result<bool, Error> {
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

    Ok(true)
}

fn file_overwriting_hexa(_path : &str, _char : u8)-> Result<bool, Error> {
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

    Ok(true)
}

fn gutmann_overwrite_file(_path : &str)-> Result<bool, Error> {
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
    Ok(true)
}

fn hmgi_s5_overwrite_file(_path : &str)-> Result<bool, Error >{
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
    return Ok(true)
}

fn rcmp_tssit_ops_ii_overwrite_file(_path : &str) -> Result<bool, Error >{
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
    Ok(true)
}

fn afssi_5020_overwrite_file(_path : &str) -> Result<bool, Error >{
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
    Ok(true)
}

fn dod_522022_mece_overwrite_file(_path : &str) -> Result<bool, Error >{
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
    Ok(true)
}

fn dod_522022_me_overwrite_file(_path : &str) -> Result<bool, Error >{
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
    Ok(true)
}

fn get_random_patern()->[u8; 3]{
    let x1:u8 = rand::random();
    let x2:u8 = rand::random(); 
    let x3:u8 = rand::random(); 
    return [x1,x2,x3]
}

fn delete_file(_path : String)-> Result<bool, Error>{
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
    
    Ok(true)
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

#[derive(Debug, Clone, Copy)]
pub enum Error{
    NotAFolderOrDidntExist,     // If the folder at the given path didn't exist or is not a folder
    NotAFileOrDidntExist,       // If the file at the given path didn't exist or is not a file
    ErrorGetFileName,           // Error when trying to retrieve the file name
    RenameError,                // Error when trying to rename the file  
    RemoveFileError,            // Error when trying to remove file
    BufferWritingError,         // Error in buffer writing function
    Dod522022MECEErrorFourth,   // Error in the fourth overwriting of DOD 522022 MECE
    Dod522022MECEErrorFifth,    // Error in the fifth overwriting of DOD 522022 MECE
    Dod522022MECEErrorSixth,    // Error in the sixth overwriting of DOD 522022 MECE
    Dod522022MECEErrorSeventh,  // Error in the seventh overwriting of DOD 522022 MECE
    Dod522022MEErrorFirst,      // Error in the first overwriting of DOD 522022 ME
    Dod522022MEErrorSecond,     // Error in the second overwriting of DOD 522022 ME
    Dod522022MEErrorThird,      // Error in the third overwriting of DOD 522022 ME
    Afssi5020ErrorFirst,        // Error in the first overwriting of AFSSI 5020
    Afssi5020ErrorSecond,       // Error in the second overwriting of AFSSI 5020
    Afssi5020ErrorThird,        // Error in the third overwriting of AFSSI 5020
    RcmpTssitOpsIIErrorFirst,   // Error in the first overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorSecond,  // Error in the second overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorThird,   // Error in the third overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorFourth,  // Error in the fourth overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorFifth,   // Error in the fifth overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorSixth,   // Error in the sixth overwriting of RCMP TSSIT OPS II
    RcmpTssitOpsIIErrorSeventh, // Error in the seventh overwriting of RCMP TSSIT OPS II
    HmgiS5ErrorFirst,           // Error in the first overwriting with HMGI S5 method
    HmgiS5ErrorSecond,          // Error in the first overwriting with HMGI S5 method
    PseudoRandomError,          // Error in the overwriting with PseudoRandom method
    FileOpeningError,           // Error in file opening
    VerificationFailed,         // One of the verification Fail
    BufferReadingError,         // Error in buffer reading
    RemoveDirError,             // Remove folder error
    GutmannRandomPaternError,   // Gutman random patern overwriting problem Error
    GutmannSpecificPaternError, // Gutman specific patern overwriting problem Error
}