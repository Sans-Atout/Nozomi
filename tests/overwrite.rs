use std::{path::Path, env, fs};

fn is_file_owerwritted(path : &str) -> bool{
    // Test if file exist
    let tested_path = Path::new(path);
    if !tested_path.exists() {
        return false;
    }

    // Test if file is overwritten or not
    let breton_text = env::var("TEXT").unwrap()+"\n";
    let tested_content = fs::read_to_string(tested_path);

    if tested_content.is_err(){
        return true;
    }

    return breton_text == tested_content.unwrap();
}

mod afssi_5020 {
    use crate::is_file_owerwritted;
    use nozomi::overwrite::afssi_5020_overwrite_file as erase_method;
    use nozomi::enums::erase_method::EraserEntity::Afssi5020 as erase_entity;
    static ERASE_METHOD_NAME: &str = "afssi_5020";


    /// Test if the overwrite method for this particular erase protocole work well or not.
    /// 
    /// This test need a valid file in `data` folder generate by `./test.sh` script.
    #[test]
    fn overwrite_method() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
        let result = erase_method(overwrite_path);
        assert!(result.is_ok());
        assert!(is_file_owerwritted(overwrite_path));
    }

    #[test]
    fn file_not_found() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/invalid.txt");
        let result = erase_method(overwrite_path);
        assert!(result.is_err());
    }

    #[test]
    fn no_write_rigth() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/write_error.txt");
        let result = erase_method(overwrite_path);
        assert!(result.is_err());
    }

    #[test]
    fn can_erase_file() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/erase_method.txt");
        let result = nozomi::erase_file(overwrite_path,erase_entity);
        assert!(result.is_ok());
    }


    #[test]
    fn erase_folder() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/folder");
        let result = nozomi::erase_folder(overwrite_path,erase_entity,false);
        assert!(result.is_ok());
    }
}
