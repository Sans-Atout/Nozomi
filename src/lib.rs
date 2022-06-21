// Librairie use in this librairie
use rand;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::Read;
use std::io;

// Erase entity
#[derive(Debug, Clone, Copy)]
pub enum EraserEntity {
    Dod522022MECE,
    Dod522022ME,
    Dod522028MSTD,
    UsArmyAr38019,
    Afssi5020,
    RcmpTssitOpsII,
    HmgiS5,
    Gutmann
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