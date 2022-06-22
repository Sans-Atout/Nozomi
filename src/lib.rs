// Librairie use in this librairie
use rand;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::Read;
use std::io;
use std::path::Path;

// Erase entity
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

pub fn erase_folder(_path : &str, erase_method : EraserEntity) -> Result<bool, io::Result<()>>{
    let _p = Path::new(_path);
    if ! (_p.exists() && _p.is_dir()){
        return Ok(false)
    }

    let paths = fs::read_dir(_path).unwrap();
    for path in paths {
        let p = path.unwrap().path();
        let _file_name = match p.to_str(){
            Some(file) => file,
            None => return Ok(false)
        };
        match erase_file(_file_name,erase_method){
            Ok(_) => true,
            Err(_) => return Ok(false)
        };

    }

    Ok(true)
}

/// Erase one file wirh a giver erase method from EraserEntity
pub fn erase_file(_path : &str, erase_method : EraserEntity) -> Result<bool, io::Result<()>>{
    let _p = Path::new(_path);
    if ! (_p.exists() && _p.is_file()){
        return Ok(false)
    }
    match erase_method{
        EraserEntity::Gutmann => {
            match gutmann_overwrite_file(_path) {
                Ok(_) => (),
                Err(_error) => return Ok(false)
            };
        },
        EraserEntity::HmgiS5 => {
            match hmgi_s5_overwrite_file(_path){
                Ok(_) => (),
                Err(_error) => return Ok(false)
            };
        },
        EraserEntity::RcmpTssitOpsII =>{
            match rcmp_tssit_ops_ii_overwrite_file(_path){
                Ok(_) => (),
                Err(_error) => return Ok(false)
            };
        },
        EraserEntity::PseudoRandom =>{
            match file_overwriting_random(_path){
                Ok(_) => (),
                Err(_error) => return Ok(false)
            }
        },
        EraserEntity::Afssi5020 => {
            match afssi_5020_overwrite_file(_path){
                Ok(_) => (),
                Err(_error) => return Ok(false)
            }
        },
        EraserEntity::Dod522022MECE => {
            match dod_522022_mece_overwrite_file(_path){
                Ok(_) => (),
                Err(_error) => return Ok(false)
            }
        },
        EraserEntity::Dod522022ME => {
            match dod_522022_me_overwrite_file(_path){
                Ok(_) => (),
                Err(_error) => return Ok(false)
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

fn file_overwriting(_path : &str, _char : [u8; 3])-> io::Result<()> {
    // Declare important variable for this 
    let mut _file = File::options().read(true).open(_path)?;
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();
    
    reader.read_to_end(&mut buffer)?;
    for _id in 0..buffer.len() { buffer_modified.push(_char[_id % 3]);}

     _file = File::options().write(true).open(_path)?;
    _file.write_all(buffer_modified.as_slice())?;

    Ok(())
}

fn file_overwriting_random(_path : &str)-> io::Result<()> {
    // Declare important variable for this 
    let mut _file = File::options().read(true).open(_path)?;
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();
    
    reader.read_to_end(&mut buffer)?;
    for _id in 0..buffer.len() {
        let mut _rand:u8 = rand::random(); 
        buffer_modified.push(_rand);
    }

     _file = File::options().write(true).open(_path)?;
    _file.write_all(buffer_modified.as_slice())?;

    Ok(())
}

fn file_overwriting_hexa(_path : &str, _char : u8)-> io::Result<()> {
    // Declare important variable for this 
    let mut _file = File::options().read(true).open(_path)?;
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();
    
    reader.read_to_end(&mut buffer)?;
    for _id in 0..buffer.len() { buffer_modified.push(_char);}

     _file = File::options().write(true).open(_path)?;
    _file.write_all(buffer_modified.as_slice())?;

    Ok(())
}

fn gutmann_overwrite_file(_path : &str)-> io::Result<()> {
    file_overwriting_random(_path)?;
    file_overwriting_random(_path)?;
    file_overwriting_random(_path)?;
    file_overwriting_random(_path)?;
    file_overwriting(_path,[0x55 as u8,0x55 as u8,0x55 as u8])?;
    file_overwriting(_path,[0xAA as u8,0xAA as u8,0xAA as u8])?;
    file_overwriting(_path,[0x92 as u8,0x49 as u8,0x24 as u8])?;
    file_overwriting(_path,[0x00 as u8,0x00 as u8,0x00 as u8])?;
    file_overwriting(_path,[0x11 as u8,0x11 as u8,0x11 as u8])?;
    file_overwriting(_path,[0x22 as u8,0x22 as u8,0x22 as u8])?;
    file_overwriting(_path,[0x33 as u8,0x33 as u8,0x33 as u8])?;
    file_overwriting(_path,[0x44 as u8,0x44 as u8,0x44 as u8])?;
    file_overwriting(_path,[0x55 as u8,0x55 as u8,0x55 as u8])?;
    file_overwriting(_path,[0x66 as u8,0x66 as u8,0x66 as u8])?;
    file_overwriting(_path,[0x77 as u8,0x77 as u8,0x77 as u8])?;
    file_overwriting(_path,[0x88 as u8,0x88 as u8,0x88 as u8])?;
    file_overwriting(_path,[0x99 as u8,0x99 as u8,0x99 as u8])?;
    file_overwriting(_path,[0xAA as u8,0xAA as u8,0xAA as u8])?;
    file_overwriting(_path,[0xBB as u8,0xBB as u8,0xBB as u8])?;
    file_overwriting(_path,[0xCC as u8,0xCC as u8,0xCC as u8])?;
    file_overwriting(_path,[0xDD as u8,0xDD as u8,0xDD as u8])?;
    file_overwriting(_path,[0xEE as u8,0xEE as u8,0xEE as u8])?;
    file_overwriting(_path,[0xFF as u8,0xFF as u8,0xFF as u8])?;
    file_overwriting(_path,[0x92 as u8,0x49 as u8,0x24 as u8])?;
    file_overwriting(_path,[0x49 as u8,0x24 as u8,0x92 as u8])?;
    file_overwriting(_path,[0x6D as u8,0xB6 as u8,0xDB as u8])?;
    file_overwriting(_path,[0xB6 as u8,0xDB as u8,0x6D as u8])?;
    file_overwriting(_path,[0xDB as u8,0x6D as u8,0xB6 as u8])?;
    file_overwriting_random(_path)?;
    file_overwriting_random(_path)?;
    file_overwriting_random(_path)?;
    file_overwriting_random(_path)?;
    Ok(())
}

fn hmgi_s5_overwrite_file(_path : &str)-> Result<bool, &'static str >{
    match file_overwriting(_path,[0x00 as u8,0x00 as u8,0x00 as u8]){
        Ok(_) => true,
        Err(_) => return Err("Error in the first pass")
    };
    match file_overwriting(_path,[0xFF as u8,0xFF as u8,0xFF as u8]){
        Ok(_) => true,
        Err(_) => return Err("Error in the second pass")
    };
    let mut _file = match File::options().read(true).open(_path){
        Ok(file) => file,
        Err(_) => return Err("Error in file opening")
    };

    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let _d = match reader.read_to_end(&mut buffer){
        Ok(_) => true,
        Err(_) => return Err("Error in buffer reading")
    };
    for _char in buffer { 
        if ! _char as u8 == 0xFF as u8{
            return Err("One of byte is not equal at 0xFF")
        }
    }
    return Ok(true)
}

fn rcmp_tssit_ops_ii_overwrite_file(_path : &str) -> Result<bool, &'static str >{
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the first pass")
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => (),
        Err(_) => return Err("Error in the second pass")
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the third pass")
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the fourth pass")
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the fifth pass")
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the sixth pass")
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err("Error in the last pass")
    };
    Ok(true)
}

fn afssi_5020_overwrite_file(_path : &str) -> Result<bool, &'static str >{
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the first pass")
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the second pass")
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err("Error in the last pass")
    };
    Ok(true)
}

fn dod_522022_mece_overwrite_file(_path : &str) -> Result<bool, &'static str >{
    match dod_522022_me_overwrite_file(_path){
        Ok(_) => true,
        Err(_) => return Err("Error in three first pass")
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the fourth pass")
    };
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the fifth pass")
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the sixth pass")
    };
    match file_overwriting_random(_path){
        Ok(_) => true,
        Err(_) => return Err("Error in the last pass")
    };
    Ok(true)
}

fn dod_522022_me_overwrite_file(_path : &str) -> Result<bool, &'static str >{
    match file_overwriting_hexa(_path,0x00 as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the first pass")
    };
    match file_overwriting_hexa(_path,0xFF as u8){
        Ok(_) => true,
        Err(_) => return Err("Error in the second pass")
    };
    match file_overwriting(_path,get_random_patern()){
        Ok(_) => true,
        Err(_) => return Err("Error in the third pass")
    };
    Ok(true)
}

fn get_random_patern()->[u8; 3]{
    let x1:u8 = rand::random();
    let x2:u8 = rand::random(); 
    let x3:u8 = rand::random(); 
    return [x1,x2,x3]
}

fn delete_file(_path : String)-> Result<bool, &'static str>{
    let mut new_path = _path.clone();
    let size = match get_file_name_size(new_path.clone()){
        Ok(s) => s,
        Err(_) => return Err("Error")
    };

    for  s in 0..size+1 {
        new_path = match rename_file(new_path, size -s ){
            Ok(file) => file,
            Err(_) => return Err("Error in renaming file")
        };

    }
    match fs::remove_file(new_path){
         Ok(_) => true,
         Err(_) => return Err("Fail to remove file")
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

fn rename_file(_path : String, size : u32) -> Result<String, &'static str>{
    let _p = Path::new(&_path);
    if ! (_p.exists() && _p.is_file()){
        return Err("File dind't exist ou the path given is not a file")
    }
    let _file_name = match _p.file_name(){
        Some(file) => file,
        None => return Err("Error")
    };
    let dir = _p.parent().unwrap().to_str().unwrap();
    let new_file_name = String::from([dir, &generate_zero_string(size).as_str()].join("/"));
    match fs::rename(&_path,&new_file_name){
        Ok(_) => true,
        Err(_) => return Err("Rename fail")
    };
    Ok(new_file_name)
}

fn get_file_name_size(new_path : String) -> Result<u32, &'static str>{
    let _p = Path::new(&new_path);
    if ! (_p.exists() && _p.is_file()){
        return Err("File dind't exist ou the path given is not a file")
    }
    let file_name = match _p.file_name(){
        Some(file) => file,
        None => return Err("Error")
    };
    Ok(file_name.len() as u32)
}