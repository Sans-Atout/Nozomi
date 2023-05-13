use error_stack::{Result, ResultExt};

use crate::error::ProcessError;
use crate::utils::Overwrite;

/// Function that implement [Gutmann overwrite method](https://en.wikipedia.org/wiki/Gutmann_method)
///
/// Argument :
/// * `path` (&str) : the file path you want to erase
///
/// Return
/// * () : if success
/// * Process Error : if fail (wrong path given or wrong right)
///
/// # Example :
/// ```
/// use nozomi::method::gutmann_overwrite_file;
///
/// fn main(){
///     gutmann_overwrite_file("/path/to/file")?;
/// }
/// ```
pub fn gutmann_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    let mut step = 1;
    let overwrite_pattern: Vec<[u8; 3]> = vec![
        [0x55_u8, 0x55_u8, 0x55_u8],
        [0xAA_u8, 0xAA_u8, 0xAA_u8],
        [0x92_u8, 0x49_u8, 0x24_u8],
        [0x49_u8, 0x24_u8, 0x92_u8],
        [0x24_u8, 0x92_u8, 0x49_u8],
        [0x00_u8, 0x00_u8, 0x00_u8],
        [0x11_u8, 0x11_u8, 0x11_u8],
        [0x22_u8, 0x22_u8, 0x22_u8],
        [0x33_u8, 0x33_u8, 0x33_u8],
        [0x44_u8, 0x44_u8, 0x44_u8],
        [0x55_u8, 0x55_u8, 0x55_u8],
        [0x66_u8, 0x66_u8, 0x66_u8],
        [0x77_u8, 0x77_u8, 0x77_u8],
        [0x88_u8, 0x88_u8, 0x88_u8],
        [0x99_u8, 0x99_u8, 0x99_u8],
        [0xAA_u8, 0xAA_u8, 0xAA_u8],
        [0xBB_u8, 0xBB_u8, 0xBB_u8],
        [0xCC_u8, 0xCC_u8, 0xCC_u8],
        [0xDD_u8, 0xDD_u8, 0xDD_u8],
        [0xEE_u8, 0xEE_u8, 0xEE_u8],
        [0xFF_u8, 0xFF_u8, 0xFF_u8],
        [0x92_u8, 0x49_u8, 0x24_u8],
        [0x49_u8, 0x24_u8, 0x92_u8],
        [0x24_u8, 0x92_u8, 0x49_u8],
        [0x6D_u8, 0xB6_u8, 0xDB_u8],
        [0xB6_u8, 0xDB_u8, 0x6D_u8],
        [0xDB_u8, 0x6D_u8, 0xB6_u8],
    ];

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}"
    ))?;
    step += 1;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}"
    ))?;
    step += 1;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}"
    ))?;
    step += 1;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}"
    ))?;
    step += 1;

    for pattern in &overwrite_pattern {
        Overwrite::new(_path)
            .pattern(pattern)
            .write()
            .attach_printable(format!(
                "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}",
            ))?;
        step += 1;
    }

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}",
    ))?;
    step += 1;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}",
    ))?;
    step += 1;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}",
    ))?;
    step += 1;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : Gutmann\nStep : {step}\npath : {_path}",
    ))?;

    Ok(())
}

/// Function that implement [HMGI S5 overwrite method](https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php)
///
/// Argument :
/// * `path` (&str) : the file path you want to erase
///
/// Return
/// * () : if success
/// * Process Error : if fail (wrong path given or wrong right)
///
/// # Example :
/// ```
/// use nozomi::method::hmgi_s5_overwrite_file;
///
/// fn main(){
///     hmgi_s5_overwrite_file("/path/to/file")?;
/// }
/// ```
pub fn hmgi_s5_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : HMGI S5 \nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : HMGI S5 \nStep : 2\npath : {_path}"
        ))?;

    Ok(())
}

/// Function that implement [RCMP TSSIT OPS II overwrite method](https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html)
///
/// Argument :
/// * `path` (&str) : the file path you want to erase
///
/// Return
/// * () : if success
/// * Process Error : if fail (wrong path given or wrong right)
///
/// # Example :
/// ```
/// use nozomi::method::rcmp_tssit_ops_ii_overwrite_file;
///
/// fn main(){
///     rcmp_tssit_ops_ii_overwrite_file("/path/to/file")?;
/// }
/// ```
pub fn rcmp_tssit_ops_ii_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 2\npath : {_path}"
        ))?;
    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 3\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 4\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 5\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 6\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 7\npath : {_path}"
    ))?;

    Ok(())
}

/// Function that implement [AFSSI 5020 overwrite method](https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020)
///
/// Argument :
/// * `path` (&str) : the file path you want to erase
///
/// Return
/// * () : if success
/// * Process Error : if fail (wrong path given or wrong right)
///
/// # Example :
/// ```
/// use nozomi::method::afssi_5020_overwrite_file;
///
/// fn main(){
///     afssi_5020_overwrite_file("/path/to/file")?;
/// }
/// ```
pub fn afssi_5020_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : AFSSI 5020 \nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  AFSSI 5020 \nStep : 2\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  AFSSI 5020 \nStep : 3\npath : {_path}"
    ))?;

    Ok(())
}

/// Function that implement [DOD 522022 MECE overwrite method](https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php)
///
/// Argument :
/// * `path` (&str) : the file path you want to erase
///
/// Return
/// * () : if success
/// * Process Error : if fail (wrong path given or wrong right)
///
/// # Example :
/// ```
/// use nozomi::method::dod_522022_mece_overwrite_file;
///
/// fn main(){
///     dod_522022_mece_overwrite_file("/path/to/file")?;
/// }
/// ```
pub fn dod_522022_mece_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : DOD 522022 MECE\nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 2\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 3\npath : {_path}"
    ))?;

    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : DOD 522022 MECE\nStep : 4\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 5\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 6\npath : {_path}"
    ))?;

    Ok(())
}


/// Function that implement [DOD 522022 ME overwrite method](https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php)
///
/// Argument :
/// * `path` (&str) : the file path you want to erase
///
/// Return
/// * () : if success
/// * Process Error : if fail (wrong path given or wrong right)
///
/// # Example :
/// ```
/// use nozomi::method::dod_522022_me_overwrite_file;
///
/// fn main(){
///     dod_522022_me_overwrite_file("/path/to/file")?;
/// }
/// ```
pub fn dod_522022_me_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .byte(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : DOD 522022 ME\nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .byte(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  DOD 522022 ME\nStep : 2\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  DOD 522022 ME\nStep : 3\npath : {_path}"
    ))?;
    Ok(())
}
