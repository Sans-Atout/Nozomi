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
    DoD522028MSTD,
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