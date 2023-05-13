use error_stack::{ Result, ResultExt};


use crate::error::ProcessError;
use crate::utils::Overwrite;

pub fn gutmann_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    let mut step = 1;
    let mut overwrite_patern: Vec<[u8; 3]> = Vec::new();
    overwrite_patern.push([0x55_u8, 0x55_u8, 0x55_u8]);
    overwrite_patern.push([0xAA_u8, 0xAA_u8, 0xAA_u8]);
    overwrite_patern.push([0x92_u8, 0x49_u8, 0x24_u8]);
    overwrite_patern.push([0x49_u8, 0x24_u8, 0x92_u8]);
    overwrite_patern.push([0x24_u8, 0x92_u8, 0x49_u8]);
    overwrite_patern.push([0x00_u8, 0x00_u8, 0x00_u8]);
    overwrite_patern.push([0x11_u8, 0x11_u8, 0x11_u8]);
    overwrite_patern.push([0x22_u8, 0x22_u8, 0x22_u8]);
    overwrite_patern.push([0x33_u8, 0x33_u8, 0x33_u8]);
    overwrite_patern.push([0x44_u8, 0x44_u8, 0x44_u8]);
    overwrite_patern.push([0x55_u8, 0x55_u8, 0x55_u8]);
    overwrite_patern.push([0x66_u8, 0x66_u8, 0x66_u8]);
    overwrite_patern.push([0x77_u8, 0x77_u8, 0x77_u8]);
    overwrite_patern.push([0x88_u8, 0x88_u8, 0x88_u8]);
    overwrite_patern.push([0x99_u8, 0x99_u8, 0x99_u8]);
    overwrite_patern.push([0xAA_u8, 0xAA_u8, 0xAA_u8]);
    overwrite_patern.push([0xBB_u8, 0xBB_u8, 0xBB_u8]);
    overwrite_patern.push([0xCC_u8, 0xCC_u8, 0xCC_u8]);
    overwrite_patern.push([0xDD_u8, 0xDD_u8, 0xDD_u8]);
    overwrite_patern.push([0xEE_u8, 0xEE_u8, 0xEE_u8]);
    overwrite_patern.push([0xFF_u8, 0xFF_u8, 0xFF_u8]);
    overwrite_patern.push([0x92_u8, 0x49_u8, 0x24_u8]);
    overwrite_patern.push([0x49_u8, 0x24_u8, 0x92_u8]);
    overwrite_patern.push([0x24_u8, 0x92_u8, 0x49_u8]);
    overwrite_patern.push([0x6D_u8, 0xB6_u8, 0xDB_u8]);
    overwrite_patern.push([0xB6_u8, 0xDB_u8, 0x6D_u8]);
    overwrite_patern.push([0xDB_u8, 0x6D_u8, 0xB6_u8]);

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

    for i in 0..overwrite_patern.len() {
        let pattern = &overwrite_patern[i];
        Overwrite::new(_path)
            .bytes_patern(pattern)
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

pub fn hmgi_s5_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : HMGI S5 \nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : HMGI S5 \nStep : 2\npath : {_path}"
        ))?;

    Ok(())
}

pub fn rcmp_tssit_ops_ii_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 2\npath : {_path}"
        ))?;
    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 3\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 4\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 5\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 6\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod : RCMP TSSIT OPS II \nStep : 7\npath : {_path}"
    ))?;

    Ok(())
}

pub fn afssi_5020_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : AFSSI 5020 \nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  AFSSI 5020 \nStep : 2\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  AFSSI 5020 \nStep : 3\npath : {_path}"
    ))?;

    Ok(())
}

pub fn dod_522022_mece_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : DOD 522022 MECE\nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 2\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 3\npath : {_path}"
    ))?;

    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : DOD 522022 MECE\nStep : 4\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 5\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  DOD 522022 MECE\nStep : 6\npath : {_path}"
    ))?;

    Ok(())
}

pub fn dod_522022_me_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    Overwrite::new(_path)
        .bytes(&0x00_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod : DOD 522022 ME\nStep : 1\npath : {_path}"
        ))?;

    Overwrite::new(_path)
        .bytes(&0xFF_u8)
        .write()
        .attach_printable(format!(
            "Overwrite method failed :\nmethod :  DOD 522022 ME\nStep : 2\npath : {_path}"
        ))?;

    Overwrite::new(_path).write().attach_printable(format!(
        "Overwrite method failed :\nmethod :  DOD 522022 ME\nStep : 3\npath : {_path}"
    ))?;
    Ok(())
}
