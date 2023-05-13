# Nozomi contribution guideline

This document serves as a reference and contains all the good practices to be applied if you wish to participate in the development of this project.

## Tables of content
1. [**Version control policy** *(vX.Y.Z)*](#version-control-policy-vxyz)
    1. [**Major version** *(vX.0.0)*](#major-version-vx00)
    2. [**Minor version** *v1.Y.0*](#minor-version-v1y0)
    3. [**Patch** *v1.1.Z*](#patch-v11z)

2. [New Erase Method](#new-erase-method)
    1. [What to check before ask for a new erase method](#what-to-check-before-ask-for-a-new-erase-method)
    2. [Change in src/method.rs file](#change-in-srcmethodrs-file)
    3. [Change in src/lib.rs file](#change-in-srclibrs-file)
    4. [Change in test.sh file](#change-in-testsh-file)
    5. [Change in tests/overwrite.rs file](#change-in-testsoverwriters-file)
3. [Other kind of contribution](#other-kind-of-contribution)
    1. [Examples](#examples)
    2. [Issues](#issues)
    3. [Spell Check](#spell-check)
    4. [Documenting](#documenting)

# **Version control policy** *(vX.Y.Z)*
## **Major version** *(vX.0.0)*
```diff
+ INFO : Each code refactorization of a new deletion method will result in a major change
```
A major release is a release that contains changes that are destructive to the user. So, if he changes to a more recent major version then he will have to modify his code 


*I hope that 2.x will be my last major version...*
### Example :
```rust
use nozomi::EraserEntity::PseudoRandom; // v1.0.2
use nozomi::OverwriteMethod::PseudoRandom; //v2.0.0
```

## **Minor version** *v1.Y.0*
```diff
+ INFO : Each addition of a new deletion method will result in a minor change
```
A minor release is a release that contains changes that are not destructive to the user. So, if he changes to a more recent he will not have to modify his code.


## **Patch** *v1.1.Z*
```diff
+ INFO : Each dependency update will result in a new patch version
! INFO : Each typo or grammar correction will not result in a new patch version
```
A dependency or internal documentation update 

# New Erase Method

## What to check before ask for a new erase method
- [ ] Does the new method I want to implement have an RFC in the issue?
- [ ] Is the RFC not being reviewed / tested / implemented?
- [ ] Is the name of my method in line with the code of conduct?
- [ ] Is the name of my method unique?
- [ ] Is my method documented somewhere (book, internet)?

All good? Then great, no worries, you can add your method!

## Change in [src/method.rs](src/method.rs) file
```rust
/// TODO change this first line
/// Function that implement [AFSSI 5020 overwrite method](https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020)
///
/// Argument :
/// * `path` (&str) : the file path you want to erase
///
/// Return
/// * () : if success
/// * Process Error : if fail (wrong path given or wrong right)
///
/// TODO Change example
/// # Example :
/// ```
/// use nozomi::method::method_name_overwrite_file;
///
/// fn main(){
///    method_name_overwrite_file("/path/to/file")?;
/// }
/// ```
pub fn method_name_overwrite_file(_path: &str) -> Result<(), ProcessError> {
    // TODO make your method here
    Ok(())
}
```

## Change in [src/lib.rs](src/lib.rs) file
```rust
...
pub enum OverwriteMethod {
    /// DOD 522022 MECE erasing method <https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php>
    Dod522022MECE,
    ...,
    /// TODO document your Overwrite method with a link
    MethodName,
    #[default]
    PseudoRandom, // NEVER change #default value for this enum
}
...
pub fn erase_file(_path: &str, erase_method: OverwriteMethod) -> Result<(), ProcessError> {
...
    match erase_method {
        OverwriteMethod::Gutmann => gutmann_overwrite_file(_path)?,
        ...,
        OverwriteMethod::Dod522022ME => dod_522022_me_overwrite_file(_path)?,
        OverwriteMethod::MethodName => method_name_overwrite_file(_path)?,
    }
...
}
```

## Change in [test.sh](test.sh) file
```shell
...
generate hmgi_s5
generate rcmp_tssit_ops_ii
generate method_name # TODO Change this to your method name
...
reverse_chmod hmgi_s5
reverse_chmod rcmp_tssit_ops_ii
reverse_chmod method_name # TODO Change this to your method name
...
```

## Change in [tests/overwrite.rs](tests/overwrite.rs) file
Add this test at the end of the test file. Do not forget to update TODO.
```rust
// TODO Change it to your method name
mod method_name {
    use crate::is_file_overwritten;
    // TODO Change the function to your overwriting method but not the alias
    use nozomi::method::method_name_overwrite_file as overwrite_method;
    // TODO Change the entity to your overwriting method but not the alias
    use nozomi::OverwriteMethod::MethodName as erase_entity;
    // TODO Change it to your overwriting algorithm
    static ERASE_METHOD_NAME: &str = "method_name";

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
        // TODO ADD POST OVERWRITE METHOD TESTING
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
```

# Other kind of contribution
## Examples
Feel free to add as many different examples as you like as long as they are not redundant.
```diff
! In the first title of the README.md remember to put the version of the library used to make the example
```
```
examples/
  |- example_1/
      |- README.md
      |- Cargo.toml
      |- src/
          |- main.rs
          |- ...
```
## Issues
/// TODO

## Spell Check
As you may have noticed, I make a lot of grammatical and spelling mistakes. Although I do everything I can to try and correct them before I post, it is possible (even certain) that some remain.

So, if you see any, please do not hesitate and thank you very much.

## Documenting
I try to do my best to ensure that the code is well documented. This also applies to README, CONTRIBUTING, etc.

If you ever feel that a part of the code is not explicit enough or lacks clarity, please feel free to open a discussion. I will try to answer as soon as possible.

To tell you the truth, this is the first open-source project I've published and I want to make it as professional as possible. So, of course, I'm open to any kind of criticism.