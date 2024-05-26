use crate::Method;
use crate::models::SecureDelete;

#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};
#[cfg(not(feature = "error-stack"))]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    secure_deletion
        .byte(&0x00_u8)
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Afssi5020, 1))?;
    secure_deletion
        .byte(&0xFF_u8)
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Afssi5020, 2))?;
    secure_deletion
        .overwrite()
        .map_err(|_| Error::OverwriteError(Method::Afssi5020, 3))?;
    Ok(secure_deletion)
}

#[cfg(test)]
#[cfg(not(feature = "error-stack"))]
mod std_test {
    use std::path::Path;

    use super::overwrite_file;
    use crate::tests::standard::{create_test_file, get_bytes};
    use crate::tests::TestType;
    use crate::Method::Afssi5020 as EraseMethod;
    use crate::{Error, Result};
    use crate::error::FSProblem;

    use pretty_assertions::{assert_eq, assert_ne};

    const METHOD_NAME: &str = "afssi_5020";

    #[test]
    fn basic_overwrite() -> Result<()> {
        let (string_path, lorem) = create_test_file(&TestType::OverwriteOnly, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        overwrite_file(&string_path)?;
        let bytes = get_bytes(&path)?;
        assert_eq!(bytes.len(), lorem.as_bytes().len());
        assert_ne!(bytes, lorem.as_bytes());
        std::fs::remove_file(&string_path).map_err(|_| Error::SystemProblem(FSProblem::Delete,string_path.to_string()))?;
        Ok(())
    }

    #[test]
    fn small_deletion() -> Result<()> {
        let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }

    #[test]
    fn medium_deletion() -> Result<()> {
        let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!Path::new(&string_path).exists());
        Ok(())
    }

    #[test]
    #[ignore = "test too long"]
    fn large_deletion() -> Result<()> {
        let (string_path, _) = create_test_file(&TestType::LargeFile, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }
    
    #[test]
    fn folder_test() -> Result<()>{
        let (string_path, _) = create_test_file(&TestType::Folder,  &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }
    #[test]
    fn permission_denied() -> Result<()>{
        let (string_path, _) = create_test_file(&TestType::WrittingError, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        let result =EraseMethod.delete(&string_path);
        println!("{:?}",result);
        assert!(result.is_err());
        let mut perms = path.metadata().unwrap().permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(&string_path, perms).map_err(|_| Error::SystemProblem(FSProblem::Permissions, string_path.clone()))?;
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }
}

// * Feature error-stack code base

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;

#[cfg(feature = "error-stack")]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    let mut secure_deletion = SecureDelete::new(path)?;
    secure_deletion
        .byte(&0x00_u8)
        .overwrite()
        .change_context(Error::OverwriteError(Method::Afssi5020, 1))?;
    secure_deletion
        .byte(&0xFF_u8)
        .overwrite()
        .change_context(Error::OverwriteError(Method::Afssi5020, 2))?;
    secure_deletion
        .overwrite()
        .change_context(Error::OverwriteError(Method::Afssi5020, 3))?;
    Ok(secure_deletion)
}


#[cfg(test)]
#[cfg(feature = "error-stack")]
mod ehanced_test {
    use std::path::Path;

    use super::overwrite_file;
    use crate::tests::ehanced::{create_test_file, get_bytes};
    use crate::tests::TestType;
    use crate::Method::Afssi5020 as EraseMethod;
    use crate::{Error, Result};
    use crate::error::FSProblem;
    use error_stack::ResultExt;

    use pretty_assertions::{assert_eq, assert_ne};

    const METHOD_NAME: &str = "afssi_5020";

    #[test]
    fn basic_overwrite() -> Result<()> {
        let (string_path, lorem) = create_test_file(&TestType::OverwriteOnly, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        overwrite_file(&string_path)?;
        let bytes = get_bytes(&path)?;
        assert_eq!(bytes.len(), lorem.as_bytes().len());
        assert_ne!(bytes, lorem.as_bytes());
        std::fs::remove_file(&string_path).change_context(Error::SystemProblem(FSProblem::Delete,string_path.to_string()))?;
        Ok(())
    }

    #[test]
    fn small_deletion() -> Result<()> {
        let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }

    #[test]
    fn medium_deletion() -> Result<()> {
        let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!Path::new(&string_path).exists());
        Ok(())
    }

    #[test]
    #[ignore = "test too long"]
    fn large_deletion() -> Result<()> {
        let (string_path, _) = create_test_file(&TestType::LargeFile, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }
    
    #[test]
    fn folder_test() -> Result<()>{
        let (string_path, _) = create_test_file(&TestType::Folder,  &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }
    #[test]
    fn permission_denied() -> Result<()>{
        let (string_path, _) = create_test_file(&TestType::WrittingError, &METHOD_NAME)?;
        let path = Path::new(&string_path);
        assert!(path.exists());
        let result =EraseMethod.delete(&string_path);
        println!("{:?}",result);
        assert!(result.is_err());
        let mut perms = path.metadata().unwrap().permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(&string_path, perms).change_context(Error::SystemProblem(FSProblem::Permissions, string_path.clone()))?;
        EraseMethod.delete(&string_path)?;
        assert!(!path.exists());
        Ok(())
    }
}
