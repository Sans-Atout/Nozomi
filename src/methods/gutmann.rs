const OVERWRITE_PATTERN: [[u8; 3]; 27] = [
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

#[cfg(not(feature = "error-stack"))]
use crate::error::{Error, Result};
use crate::methods::Method;
use crate::models::SecureDelete;

#[cfg(not(feature = "error-stack"))]
pub fn gutmann_overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 1))?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 2))?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 3))?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, 4))?;

    let mut step = 4;
    for pattern in &OVERWRITE_PATTERN {
        step += 1;
        secure_deletion
            .pattern(pattern)
            .overwrite()
            .map_err(|_| Error::OverwriteError(Method::Gutmann, step))?;
    }
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 1))?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 2))?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 3))?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Gutmann, step + 4))?;
    Ok(secure_deletion)
}

#[cfg(test)]
#[cfg(not(feature = "error-stack"))]
mod std_test {
    use std::fs::create_dir_all;

    use super::gutmann_overwrite_file;
    use crate::tests::standard::{file, get_bytes};
    use crate::{Error, Result};

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn basic_overwrite() -> Result<()> {
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("gutmann");
        if !tmp_file.as_path().exists() {
            create_dir_all(&tmp_file).map_err(|e| Error::FileCreationError(e))?;
        }
        tmp_file.push("std_basic_overwrite");
        tmp_file.set_extension("txt");
        let path = tmp_file.as_path();
        file(path, "Hello, world!")?;
        assert!(path.exists());
        let str = path.to_str().ok_or(Error::StringConversionError)?;
        gutmann_overwrite_file(str)?;
        let bytes = get_bytes(&path)?;
        assert_eq!(bytes.len(), b"Hello, world!".len());
        assert_ne!(bytes, b"Hello, world!");
        Ok(())
    }
}
