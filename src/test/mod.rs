pub mod overwrite;


#[cfg(test)]
mod test{
    use std::path::Path;

    use crate::{utils::{generate_zero_string, delete_file}, erase_file, enums::erase_method::EraserEntity};

    #[test]
    fn zero_string(){
        let string_res = generate_zero_string(4);
        assert_eq!(string_res,"00000".to_string());
    }

    #[test]
    fn delete(){
        let mut result = delete_file("./data/delete_file.txt".to_string());
        assert!(result.is_ok());
        assert!(!std::path::Path::new("./data/delete_file.txt").exists());
        result = delete_file("./data/delete_file.txt".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn file(){
        let mut result = erase_file(&"./data/file_1.txt",EraserEntity::PseudoRandom);
        assert!(result.is_ok());
        assert!(!Path::new("./data/file_1.txt").exists());
        result = erase_file(&"./data/file_1.txt",EraserEntity::PseudoRandom);
        assert!(result.is_err());
    }

    #[test]
    fn folder(){
        let mut result = crate::erase_folder(&"./data/folder",EraserEntity::PseudoRandom, false);
        assert!(result.is_ok());
        assert!(!Path::new("./data/folder").exists());
        result = crate::erase_folder(&"./data/folder",EraserEntity::PseudoRandom, false);
        assert!(result.is_err());
    }
}