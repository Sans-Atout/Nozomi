use crate::{Error, Result};
use std::io::prelude::*;
use std::{fs::File, io::Write, path::Path};

pub fn file(path: &Path, lorem: &str) -> Result<()> {
    let mut file = File::create(path).map_err(|e| Error::FileCreationError(e))?;
    file.write_all(lorem.as_bytes())
        .map_err(|e| Error::FileCreationError(e))?;
    Ok(())
}

pub fn get_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut created_file = File::open(path).map_err(|e| Error::FileCreationError(e))?;
    let mut data = vec![];
    created_file
        .read_to_end(&mut data)
        .map_err(|e| Error::FileCreationError(e))?;
    Ok(data)
}

mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create() -> Result<()> {
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("nozomi_tmp_file_creation_test");
        let path = tmp_file.as_path();
        file(path, "Hello, world!")?;
        assert!(path.exists());
        let mut created_file = File::open(path).map_err(|e| Error::FileCreationError(e))?;
        let mut created_file_content = String::new();
        created_file
            .read_to_string(&mut created_file_content)
            .map_err(|e| Error::FileCreationError(e))?;
        assert_eq!(created_file_content, "Hello, world!");
        Ok(())
    }

    #[test]
    fn bytes_assertion() -> Result<()> {
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("nozomi_tmp_file_creation_test");
        let path = tmp_file.as_path();
        file(&path, "Hello, world!")?;
        assert!(path.exists());
        assert_eq!(get_bytes(&path)?, b"Hello, world!");
        Ok(())
    }
}
