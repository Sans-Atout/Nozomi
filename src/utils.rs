use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::Path,
};

use error_stack::{IntoReport, Report, Result, ResultExt};

use crate::error::{InputError, ProcessError};

pub struct Overwrite {
    path: String,
    bytes_patern: Option<[u8; 3]>,
    bytes: Option<u8>,
    random: bool,
}

impl Overwrite {
    pub fn new(path: &str) -> Overwrite {
        Overwrite {
            path: path.to_string(),
            bytes_patern: None,
            bytes: None,
            random: false,
        }
    }

    pub fn write(&self) -> Result<(), ProcessError> {
        let mut _file = File::options()
            .read(true)
            .open(&self.path)
            .into_report()
            .change_context(InputError)
            .attach_printable(format!("Invalid reading rigth for file : {}", self.path))
            .change_context(ProcessError).attach_printable("Process Failled : overwriting failed".to_owned(),)?;

        let mut reader = BufReader::new(_file);
        let mut buffer = Vec::new();
        let mut buffer_modified = Vec::new();

        reader
            .read_to_end(&mut buffer)
            .into_report()
            .change_context(InputError)
            .attach_printable(format!("buffer reading error for file : {}", self.path))

            .change_context(ProcessError).attach_printable("Process Failled : overwriting failed".to_owned(),)?;

        if self.random {
            for _id in 0..buffer.len() {
                let mut _rand: u8 = rand::random();
                buffer_modified.push(_rand);
            }
        }

        if self.bytes.is_some(){
            let data = self.bytes.unwrap();
            for _id in 0..buffer.len() {
                buffer_modified.push(data);
            }
        
        }

        if self.bytes_patern.is_some(){
            let data = self.bytes_patern.unwrap();
            for _id in 0..buffer.len() {
                buffer_modified.push(data[_id % 3]);
            }
        }

        _file = File::options()
            .write(true)
            .open(&self.path)
            .into_report()
            .change_context(InputError)
            .attach_printable(format!("Invalid writing rigth for file :  {}", self.path))

            .change_context(ProcessError).attach_printable("Process Failled : overwriting failed".to_owned(),)?;

        _file
            .write_all(buffer_modified.as_slice())
            .into_report()
            .change_context(InputError)
            .attach_printable(format!("buffer reading error for file : {}", self.path))
            .change_context(ProcessError).attach_printable("Process Failled : overwriting failed".to_owned(),)?;

        Ok(())
    }

    pub fn bytes(&mut self, bytes: &u8) -> &mut Overwrite {
        self.bytes = Some(bytes.clone());
        self.random = false;
        return self;
    }

    pub fn bytes_patern(&mut self, bytes_patern: &[u8; 3]) -> &mut Overwrite {
        self.bytes_patern = Some(bytes_patern.clone());
        self.random = false;
        return self;
    }
}

pub fn delete_file(_path: String) -> Result<(), InputError> {
    let mut new_path = _path.clone();
    let size = get_file_name_size(&_path)?;

    for s in 0..size + 1 {
        new_path = rename_file(new_path, size - s)?;
    }
    fs::remove_file(new_path)
        .into_report()
        .change_context(InputError)
        .attach_printable(format!("Can not delete file : {_path}"))?;

    Ok(())
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

pub fn rename_file(_path: String, size: u32) -> Result<String, InputError> {
    let _p = Path::new(&_path);

    if !_p.exists() {
        return Err(
            Report::new(InputError).attach_printable(format!("File does not exist : {_path}"))
        );
    }
    if !_p.is_file() {
        return Err(
            Report::new(InputError).attach_printable(format!("Given path is not a file : {_path}"))
        );
    }

    let _file_name = match _p.file_name() {
        Some(file) => file,
        None => {
            return Err(Report::new(InputError)
                .attach_printable(format!("Cannot retrive file name : {_path}")))
        }
    };
    let dir = _p.parent().unwrap().to_str().unwrap();
    let new_file_name = [dir, (generate_zero_string(size).as_str())].join("/");
    fs::rename(&_path, &new_file_name)
        .into_report()
        .change_context(InputError)
        .attach_printable(format!("Cannot rename file : {_path} to {new_file_name}"))?;
    Ok(new_file_name)
}

/// Function that return the size of the file name
///
/// Argument :
/// * path (&String) : The file for which you want to retrieve the name length
///
/// Return :
/// * (u32) : the name length if sucess
/// * (nozomi::error) : errors if fails
///
/// # Example:
/// ```
/// let size = match get_file_name_size(&_path) {
///     Ok(s) => s,
///     Err(error) => return Err(error),
/// };
/// ```
fn get_file_name_size(path: &String) -> Result<u32, InputError> {
    let _p = Path::new(path);
    if !_p.exists() {
        return Err(
            Report::new(InputError).attach_printable(format!("File does not exist : {path}"))
        );
    }
    if !_p.is_file() {
        return Err(
            Report::new(InputError).attach_printable(format!("Given path is not a file : {path}"))
        );
    }

    let file_name = match _p.file_name() {
        Some(file) => file,
        None => {
            return Err(Report::new(InputError)
                .attach_printable(format!("Cannot retrive file name : {path}")))
        }
    };
    Ok(file_name.len() as u32)
}
