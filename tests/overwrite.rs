use std::{env, fs, path::Path};

/// Method only use in the test scope
///
/// This method checks if the file was overwritten or not
/// It's didn't check if the overwrite method overwrites with the good pattern or not
///
/// Argument :
/// * `path` (&str) : the path to test
///
/// Return :
/// * (boolean) : is the file overwritten or not
///
/// # Example :
/// ```rust
/// use crate::is_file_overwritten;
///
/// #[test]
/// fn overwrite_method() {
///    let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
///    let result = overwrite_method(overwrite_path);
///    assert!(is_file_overwritten(overwrite_path));
/// }
/// ```
fn is_file_overwritten(path: &str) -> bool {
    // Test if file exist
    let tested_path = Path::new(path);
    if !tested_path.exists() {
        return false;
    }

    // Test if file is overwritten or not
    let breton_text = env::var("TEXT").unwrap() + "\n";
    let tested_content = fs::read_to_string(tested_path);

    if tested_content.is_err() {
        return true;
    }

    breton_text == tested_content.unwrap()
}

/// Test for the afssi_5020 erase method :
/// * is file well overwritten
/// * is function return proper error if file not found
/// * is function return proper error if the file has wrong write right
/// * is the erase folder function work with this method
/// * is the erase file function work with this method
mod afssi_5020 {
    use crate::is_file_overwritten;
    // Change the function to your overwriting method but not the alias
    use nozomi::method::afssi_5020_overwrite_file as overwrite_method;
    // Change the entity to your overwriting method but not the alias
    use nozomi::OverwriteMethod::Afssi5020 as erase_entity;
    // Change it to your overwriting algorithm
    static ERASE_METHOD_NAME: &str = "afssi_5020";

    /// Test if the overwrite method for this particular erase protocol work well or not.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    /// This is the only method you needs to change if you want to add a proper erase method
    ///
    /// Test success is all three condition is met :
    /// * function overwrite_method is success
    /// * file is overwritten
    /// * file is overwritten with good method
    #[test]
    fn overwrite() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_ok());
        assert!(is_file_overwritten(overwrite_path));
        // TODO add file overwritten method check
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the file is not found.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn file_not_found() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/invalid.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the user.
    /// does not have proper right on the file
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn no_write_right() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/write_error.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_file_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_file() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/erase_method.txt");
        let result = nozomi::erase_file(overwrite_path, erase_entity);
        assert!(result.is_ok());
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_folder_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn erase_folder() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/folder");
        let result = nozomi::erase_folder(overwrite_path, erase_entity, false);
        assert!(result.is_ok());
    }
}

/// Test for the dod_522022_me erase method :
/// * is file well overwritten
/// * is function return proper error if file not found
/// * is function return proper error if the file has wrong write right
/// * is the erase folder function work with this method
/// * is the erase file function work with this method
mod dod_522022_me {
    use crate::is_file_overwritten;
    use nozomi::{erase_file, erase_folder};
    // Change the function to your overwriting method but not the alias
    use nozomi::method::dod_522022_me_overwrite_file as overwrite_method;
    // Change the entity to your overwriting method but not the alias
    use nozomi::OverwriteMethod::Dod522022ME as erase_entity;
    // Change it to your overwriting algorithm
    static ERASE_METHOD_NAME: &str = "dod_522022_me";

    /// Test if the overwrite method for this particular erase protocol work well or not.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    /// This is the only method you needs to change if you want to add a proper erase method
    ///
    /// Test success is all three condition is met :
    /// * function overwrite_method is success
    /// * file is overwritten
    /// * file is overwritten with good method
    #[test]
    fn overwrite() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
        println!("{}", overwrite_path);
        let result = overwrite_method(overwrite_path);
        assert!(result.is_ok());
        assert!(is_file_overwritten(overwrite_path));
        // TODO add file overwritten method check
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the file is not found.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn file_not_found() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/invalid.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the user.
    /// does not have proper right on the file
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn no_write_right() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/write_error.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_file_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_file() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/erase_method.txt");
        let result = erase_file(overwrite_path, erase_entity);
        assert!(result.is_ok())
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_folder_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_folder() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/folder");
        let result = erase_folder(overwrite_path, erase_entity, false);
        assert!(result.is_ok());
    }
}

/// Test for the dod_522022_mece erase method :
/// * is file well overwritten
/// * is function return proper error if file not found
/// * is function return proper error if the file has wrong write right
/// * is the erase folder function work with this method
/// * is the erase file function work with this method
mod dod_522022_mece {
    use crate::is_file_overwritten;
    use nozomi::{erase_file, erase_folder};
    // Change the function to your overwriting method but not the alias
    use nozomi::method::dod_522022_mece_overwrite_file as overwrite_method;
    // Change the entity to your overwriting method but not the alias
    use nozomi::OverwriteMethod::Dod522022MECE as erase_entity;
    // Change it to your overwriting algorithm
    static ERASE_METHOD_NAME: &str = "dod_522022_mece";

    /// Test if the overwrite method for this particular erase protocol work well or not.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    /// This is the only method you needs to change if you want to add a proper erase method
    ///
    /// Test success is all three condition is met :
    /// * function overwrite_method is success
    /// * file is overwritten
    /// * file is overwritten with good method
    #[test]
    fn overwrite() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
        println!("{}", overwrite_path);
        let result = overwrite_method(overwrite_path);
        assert!(result.is_ok());
        assert!(is_file_overwritten(overwrite_path));
        // TODO add file overwritten method check
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the file is not found.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn file_not_found() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/invalid.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the user.
    /// does not have proper right on the file
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn no_write_right() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/write_error.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_file_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_file() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/erase_method.txt");
        let result = erase_file(overwrite_path, erase_entity);
        assert!(result.is_ok())
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_folder_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_folder() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/folder");
        let result = erase_folder(overwrite_path, erase_entity, false);
        assert!(result.is_ok());
    }
}

/// Test for the gutmann erase method :
/// * is file well overwritten
/// * is function return proper error if file not found
/// * is function return proper error if the file has wrong write right
/// * is the erase folder function work with this method
/// * is the erase file function work with this method
mod gutmann {
    use crate::is_file_overwritten;
    use nozomi::{erase_file, erase_folder};
    // Change the function to your overwriting method but not the alias
    use nozomi::method::gutmann_overwrite_file as overwrite_method;
    // Change the entity to your overwriting method but not the alias
    use nozomi::OverwriteMethod::Gutmann as erase_entity;
    // Change it to your overwriting algorithm
    static ERASE_METHOD_NAME: &str = "gutmann";

    /// Test if the overwrite method for this particular erase protocol work well or not.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    /// This is the only method you needs to change if you want to add a proper erase method
    ///
    /// Test success is all three condition is met :
    /// * function overwrite_method is success
    /// * file is overwritten
    /// * file is overwritten with good method
    #[test]
    fn overwrite() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
        println!("{}", overwrite_path);
        let result = overwrite_method(overwrite_path);
        assert!(result.is_ok());
        assert!(is_file_overwritten(overwrite_path));
        // TODO add file overwritten method check
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the file is not found.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn file_not_found() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/invalid.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the user.
    /// does not have proper right on the file
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn no_write_right() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/write_error.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_file_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_file() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/erase_method.txt");
        let result = erase_file(overwrite_path, erase_entity);
        assert!(result.is_ok())
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_folder_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_folder() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/folder");
        let result = erase_folder(overwrite_path, erase_entity, false);
        assert!(result.is_ok());
    }
}

/// Test for the hmgi_s5 erase method :
/// * is file well overwritten
/// * is function return proper error if file not found
/// * is function return proper error if the file has wrong write right
/// * is the erase folder function work with this method
/// * is the erase file function work with this method
mod hmgi_s5 {
    use crate::is_file_overwritten;
    use nozomi::{erase_file, erase_folder};
    // Change the function to your overwriting method but not the alias
    use nozomi::method::gutmann_overwrite_file as overwrite_method;
    // Change the entity to your overwriting method but not the alias
    use nozomi::OverwriteMethod::HmgiS5 as erase_entity;
    // Change it to your overwriting algorithm
    static ERASE_METHOD_NAME: &str = "hmgi_s5";

    /// Test if the overwrite method for this particular erase protocol work well or not.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    /// This is the only method you needs to change if you want to add a proper erase method
    ///
    /// Test success is all three condition is met :
    /// * function overwrite_method is success
    /// * file is overwritten
    /// * file is overwritten with good method
    #[test]
    fn overwrite() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
        println!("{}", overwrite_path);
        let result = overwrite_method(overwrite_path);
        assert!(result.is_ok());
        assert!(is_file_overwritten(overwrite_path));
        // TODO add file overwritten method check
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the file is not found.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn file_not_found() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/invalid.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the user.
    /// does not have proper right on the file
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn no_write_right() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/write_error.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_file_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_file() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/erase_method.txt");
        let result = erase_file(overwrite_path, erase_entity);
        assert!(result.is_ok())
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_folder_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_folder() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/folder");
        let result = erase_folder(overwrite_path, erase_entity, false);
        assert!(result.is_ok());
    }
}

/// Test for the rcmp_tssit_ops_ii erase method :
/// * is file well overwritten
/// * is function return proper error if file not found
/// * is function return proper error if the file has wrong write right
/// * is the erase folder function work with this method
/// * is the erase file function work with this method
mod rcmp_tssit_ops_ii {
    use crate::is_file_overwritten;
    use nozomi::{erase_file, erase_folder};
    // Change the function to your overwriting method but not the alias
    use nozomi::method::rcmp_tssit_ops_ii_overwrite_file as overwrite_method;
    // Change the entity to your overwriting method but not the alias
    use nozomi::OverwriteMethod::RcmpTssitOpsII as erase_entity;
    // Change it to your overwriting algorithm
    static ERASE_METHOD_NAME: &str = "rcmp_tssit_ops_ii";

    /// Test if the overwrite method for this particular erase protocol work well or not.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    /// This is the only method you needs to change if you want to add a proper erase method
    ///
    /// Test success is all three condition is met :
    /// * function overwrite_method is success
    /// * file is overwritten
    /// * file is overwritten with good method
    #[test]
    fn overwrite() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/over_write.txt");
        println!("{}", overwrite_path);
        let result = overwrite_method(overwrite_path);
        assert!(result.is_ok());
        assert!(is_file_overwritten(overwrite_path));
        // TODO add file overwritten method check
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the file is not found.
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn file_not_found() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/invalid.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol return an error if the user.
    /// does not have proper right on the file
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function returns a proper error.
    #[test]
    fn no_write_right() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/write_error.txt");
        let result = overwrite_method(overwrite_path);
        assert!(result.is_err());
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_file_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_file() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/erase_method.txt");
        let result = erase_file(overwrite_path, erase_entity);
        assert!(result.is_ok())
    }

    /// Test if the overwrite method for this particular erase protocol is implemented in erase_folder_method
    ///
    /// This test needs a valid file in `data` folder generate by `./test.sh` script.
    ///
    /// ! Do not change change this function if you want to test your own overwriting method
    ///
    /// Test success is the function return a success
    #[test]
    fn can_erase_folder() {
        let overwrite_path = &format!("data/{ERASE_METHOD_NAME}/folder");
        let result = erase_folder(overwrite_path, erase_entity, false);
        assert!(result.is_ok());
    }
}
