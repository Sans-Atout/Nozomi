use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::Path,
};

use error_stack::{Report, Result, ResultExt};

use crate::error::{InputError, ProcessError};

/// Object that represent an overwrite task
///
/// # Example
/// ```
/// Overwrite::new(_path)
///     .bytes_pattern(pattern)
///     .write()
///     .attach_printable(format!(
///         "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}",
///     ))?;
/// ```
pub struct Overwrite {
    path: String,
    pattern: Option<[u8; 3]>,
    byte: Option<u8>,
    random: bool,
}

impl Overwrite {
    /// Function that return a new instance of an Overwrite tasks
    ///
    /// By default, the overwrite tasks is set to a random overwriting procedure. \
    /// When the overwriting task is params as you wish you have to use the write method.
    ///
    /// ## Argument :
    /// * `path` (&str) : path of the file you want to overwrite
    ///
    /// ## Return :
    /// * (Overwrite) : an overwrite task object with random as overwrite method
    ///
    /// # Example
    /// ```
    /// let ow_task = Overwrite::new(_path);
    /// ```
    pub fn new(path: &str) -> Overwrite {
        Overwrite {
            path: path.to_string(),
            pattern: None,
            byte: None,
            random: true,
        }
    }

    /// Function that execute the overwrite task
    ///
    /// Possible overwriting scheme :
    /// * default : random u8 value
    /// * byte : a defined u8 value
    /// * bytes pattern : a defined array of 3 u8 values
    ///
    /// ## Return :
    /// * () : everything is ok
    /// * ProcessError : an process error with an detail stack of what happen (I hope so)
    ///
    /// # Example
    /// Default method
    /// ```rust
    /// Overwrite::new(_path).write()?;
    /// ```
    /// byte method
    /// ```rust
    /// Overwrite::new(_path).byte(0x42_u8).write()?;
    /// ```
    /// byte pattern method
    /// ```rust
    /// Overwrite::new(_path).pattern([0x00_u8, 0x42_u8, 0xFF_u8]).write()?;
    /// ```
    pub fn write(&self) -> Result<(), ProcessError> {
        let mut _file = File::options()
            .read(true)
            .open(&self.path)
            .change_context(InputError)
            .attach_printable(format!("Invalid reading right for file : {}", self.path))
            .change_context(ProcessError)
            .attach_printable("Process Failed : overwriting failed".to_owned())?;

        let mut reader = BufReader::new(_file);
        let mut buffer = Vec::new();
        let mut buffer_modified = Vec::new();

        reader
            .read_to_end(&mut buffer)
            .change_context(InputError)
            .attach_printable(format!("buffer reading error for file : {}", self.path))
            .change_context(ProcessError)
            .attach_printable("Process Failed : overwriting failed".to_owned())?;

        if self.random {
            for _id in 0..buffer.len() {
                let mut _rand: u8 = rand::random();
                buffer_modified.push(_rand);
            }
        }

        if self.byte.is_some() {
            let data = self.byte.unwrap();
            for _id in 0..buffer.len() {
                buffer_modified.push(data);
            }
        }

        if self.pattern.is_some() {
            let data = self.pattern.unwrap();
            for _id in 0..buffer.len() {
                buffer_modified.push(data[_id % 3]);
            }
        }

        _file = File::options()
            .write(true)
            .open(&self.path)
            .change_context(InputError)
            .attach_printable(format!("Invalid writing right for file :  {}", self.path))
            .change_context(ProcessError)
            .attach_printable("Process Failed : overwriting failed".to_owned())?;

        _file
            .write_all(buffer_modified.as_slice())
            .change_context(InputError)
            .attach_printable(format!("buffer reading error for file : {}", self.path))
            .change_context(ProcessError)
            .attach_printable("Process Failed : overwriting failed".to_owned())?;

        Ok(())
    }

    /// Function that configure an existing Overwriting instance to make sure that
    /// the file will be overwrite with an given byte.
    ///
    /// When the overwriting task is params as you wish you have to use the write method.
    ///
    /// ## Argument :
    /// * byte (u8) : Byte you want to use to replace all bytes  in the file.
    ///
    /// ## Return :
    /// * () : everything is ok
    /// * ProcessError : an process error with an detail stack of what happen (I hope so)
    ///
    /// # Example
    /// ```
    /// Overwrite::new(_path).byte(&0x42_u8).write()?;
    /// ```
    pub fn byte(&mut self, byte: &u8) -> &mut Overwrite {
        self.byte = Some(*byte);
        self.random = false;
        self
    }

    /// Function that configure an existing Overwriting instance to make sure that
    /// the file will be overwrite with an given byte pattern.
    ///
    /// When the overwriting task is params as you wish you have to use the write method.
    ///
    /// ## Argument :
    /// * bytes_pattern (&[u8; 3]) : Byte pattern you want to use to replace all bytes in the file.
    ///
    /// ## Return :
    /// * () : everything is ok
    /// * ProcessError : an process error with an detail stack of what happen (I hope so)
    ///
    /// # Example
    /// ```
    /// Overwrite::new(_path).pattern([0x00_u8, 0x42_u8, 0xFF_u8]).write()?;
    /// ```
    pub fn pattern(&mut self, bytes_pattern: &[u8; 3]) -> &mut Overwrite {
        self.pattern = Some(*bytes_pattern);
        self.random = false;
        self
    }
}

/// Function that delete a file.
///
/// ## Argument :
/// * _path (&str) : the path of the file you want to delete.
///
/// ## Return :
/// * () : everything is ok
/// * InputError : an input error with an detail stack of what happen (I hope so)
///
/// # Example
/// ```
/// delete_file("/path/to/file")?;
/// ```
pub fn delete_file(_path: String) -> Result<(), InputError> {
    let mut new_path = _path.clone();
    let size = get_file_name_size(&_path)?;

    for s in 0..size + 1 {
        new_path = rename_file(new_path, size - s)?;
    }
    fs::remove_file(new_path)
        .change_context(InputError)
        .attach_printable(format!("Can not delete file : {_path}"))?;

    Ok(())
}

/// Function that generate a string of a given length.
///
/// ## Argument :
/// * size (u32) : the size of the string you want to generate
///
/// ## Return :
/// * _string (String) : a string of zero of a given length
///
/// # Example
/// ```
/// generate_zero_string("/path/to/file")?;
/// ```
pub fn generate_zero_string(size: u32) -> String {
    let mut _string = String::from("");
    let mut s_size = 0;
    while s_size <= size {
        _string += "0";
        s_size += 1;
    }
    _string
}

/// Function that generate a string of a given length.
///
/// ## Argument :
/// * size (u32) : the size of the string you want to generate
/// * _path (String) : the size of the string you want to generate
///
/// ## Return :
/// * new_file_name (String) : if success the new name of a file
/// * InputError : if fail (aka wrong path given, not a file or wrong right)
///
/// # Example
/// ```
/// rename_file("/path/to/string")?;
/// ```
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
                .attach_printable(format!("Cannot retrieve file name : {_path}")));
        }
    };
    let dir = _p.parent().unwrap().to_str().unwrap();
    let new_file_name = [dir, (generate_zero_string(size).as_str())].join("/");
    fs::rename(&_path, &new_file_name)
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
/// * (u32) : the name length if success
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
                .attach_printable(format!("Cannot retrieve file name : {path}")));
        }
    };
    Ok(file_name.len() as u32)
}
