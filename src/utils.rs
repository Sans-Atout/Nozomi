use std::{fs::{File, self}, io::{Write, BufReader, Read}, path::Path};

use crate::enums::{error::Error, success::Success};


pub fn file_overwriting(_path: &str, _char: [u8; 3]) -> Result<Success, Error> {
    // Declare important variable for this
    let mut _file = match File::options().read(true).open(_path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError),
    };
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();

    let _d = match reader.read_to_end(&mut buffer) {
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError),
    };

    for _id in 0..buffer.len() {
        buffer_modified.push(_char[_id % 3]);
    }

    _file = match File::options().write(true).open(_path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError),
    };
    match _file.write_all(buffer_modified.as_slice()) {
        Ok(_) => true,
        Err(_) => return Err(Error::BufferWritingError),
    };

    Ok(Success::FileOverWriting)
}

pub fn file_overwriting_random(_path: &str) -> Result<Success, Error> {
    // Declare important variable for this
    let mut _file = match File::options().read(true).open(_path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError),
    };
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();

    let _d = match reader.read_to_end(&mut buffer) {
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError),
    };

    for _id in 0..buffer.len() {
        let mut _rand: u8 = rand::random();
        buffer_modified.push(_rand);
    }

    _file = match File::options().write(true).open(_path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError),
    };
    match _file.write_all(buffer_modified.as_slice()) {
        Ok(_) => true,
        Err(_) => return Err(Error::BufferWritingError),
    };

    Ok(Success::FileOverWriting)
}

pub fn file_overwriting_hexa(_path: &str, _char: u8) -> Result<Success, Error> {
    // Declare important variable for this
    let mut _file = match File::options().read(true).open(_path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError),
    };
    let mut reader = BufReader::new(_file);
    let mut buffer = Vec::new();
    let mut buffer_modified = Vec::new();

    let _d = match reader.read_to_end(&mut buffer) {
        Ok(_) => true,
        Err(_) => return Err(Error::BufferReadingError),
    };

    for _id in 0..buffer.len() {
        buffer_modified.push(_char);
    }

    _file = match File::options().write(true).open(_path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpeningError),
    };
    match _file.write_all(buffer_modified.as_slice()) {
        Ok(_) => true,
        Err(_) => return Err(Error::BufferWritingError),
    };

    Ok(Success::FileOverWriting)
}

pub fn get_random_patern() -> [u8; 3] {
    let x1: u8 = rand::random();
    let x2: u8 = rand::random();
    let x3: u8 = rand::random();
    [x1, x2, x3]
}

pub fn delete_file(_path: String) -> Result<Success, Error> {
    let mut new_path = _path.clone();
    let size = match get_file_name_size(&_path) {
        Ok(s) => s,
        Err(error) => return Err(error),
    };

    for s in 0..size + 1 {
        new_path = match rename_file(new_path, size - s) {
            Ok(file) => file,
            Err(_) => return Err(Error::RenameError),
        };
    }
    match fs::remove_file(new_path) {
        Ok(_) => true,
        Err(_) => return Err(Error::RemoveFileError),
    };

    Ok(Success::DeleteFileSuccess)
}

pub fn generate_zero_string(size: u32) -> String {
    let mut _string = String::from("");
    let mut s_size = 0;
    while s_size <= size {
        _string += "0";
        s_size += 1;
    }
    _string
}

pub fn rename_file(_path: String, size: u32) -> Result<String, Error> {
    let _p = Path::new(&_path);
    if !(_p.exists() && _p.is_file()) {
        return Err(Error::NotAFileOrDidntExist);
    }
    let _file_name = match _p.file_name() {
        Some(file) => file,
        None => return Err(Error::ErrorGetFileName),
    };
    let dir = _p.parent().unwrap().to_str().unwrap();
    let new_file_name = [dir, (generate_zero_string(size).as_str())].join("/");
    match fs::rename(&_path, &new_file_name) {
        Ok(_) => true,
        Err(_) => return Err(Error::RenameError),
    };
    Ok(new_file_name)
}

fn get_file_name_size(new_path: &String) -> Result<u32, Error> {
    let _p = Path::new(new_path);
    if !(_p.exists() && _p.is_file()) {
        return Err(Error::NotAFileOrDidntExist);
    }
    let file_name = match _p.file_name() {
        Some(file) => file,
        None => return Err(Error::ErrorGetFileName),
    };
    Ok(file_name.len() as u32)
}

