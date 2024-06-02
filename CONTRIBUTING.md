# Nozomi contribution guideline

This document serves as a reference and contains all the good practices to be applied if you wish to participate in the development of this project.

## Tables of content
1. [**Version control policy** *(vX.Y.Z)*](#version-control-policy-vxyz)
    1. [**Major version** *(vX.0.0)*](#major-version-vx00)
    2. [**Minor version** *v1.Y.0*](#minor-version-v1y0)
    3. [**Patch** *v1.1.Z*](#patch-v11z)

2. [New Erase Method](#new-erase-method)
    1. [What to check before ask for a new erase method](#what-to-check-before-ask-for-a-new-erase-method)
    2. [Modification process](#modification-process)
    3. [New method file template](#new-method-file-template)
    4. [Update in src/methods/mod.rs](#update-in-srcmethodsmodrs)
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

## Modification process
1) Add your methods as a rs file in [src/methods/](src/methods/) folder (cf. )
2) Fill the template with your logic ([cf. ](#add-new-method-in-enum))
3) Add new method in the [src/methods/mod.rs](src/methods/mod.rs) in Method enum ([cf.](#add-new-method-in-enum))
4) Update display trait ([cf.](#update-method-display-trait))
5) Update delete file function([cf.](#update-delete-file-function))
5) Update delete folder function([cf.](#update-delete-folder-function))

## New method file template

```rust
use crate::models::SecureDelete;
use crate::Method;

// -- Region : feature import
#[cfg(not(feature = "error-stack"))]
use crate::{Error, Result};

#[cfg(feature = "log")]
use log::info;

#[cfg(feature = "error-stack")]
use crate::{Error, Result};
#[cfg(feature = "error-stack")]
use error_stack::ResultExt;


// -- Region : Pseudo Random overwriting method for basic error handling method

#[cfg(not(feature = "error-stack"))]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    // TODO : Add your logic here
    Ok(secure_deletion)
}

// -- Region : Pseudo Random overwriting method for error-stack error handling method

#[cfg(feature = "error-stack")]
pub fn overwrite_file(path: &str) -> Result<SecureDelete> {
    // TODO : Add your logic here
    Ok(secure_deletion)
}

// -- Region : Tests 
#[cfg(test)]
mod test {
    
    const METHOD_NAME: &str = "pseudo_random"; // TODO : UPDATE HERE

    use crate::Method::PseudoRandom as EraseMethod; // TODO : UPDATE HERE

    // ! DO NOT CHANGE THE CODE BELLOW THIS POINT
    use super::overwrite_file;
    use crate::error::FSProblem;
    use crate::tests::TestType;

    /// Module containing all the tests for the standard error handling method
    #[cfg(not(feature = "error-stack"))]
    mod standard {
        use super::*;

        use crate::tests::standard::{create_test_file, get_bytes};
        use crate::{Error, Result};

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;

            use super::*;

            /// Test if the overwrite method for this particular erase protocol work well or not.
            ///
            /// Test success is all conditions are met :
            /// * function overwrite_file is success
            /// * file is overwritten
            /// * file is overwritten with good method
            /// * file is well deleted
            #[test]
            fn basic_overwrite() -> Result<()> {
                let (string_path, lorem) =
                    create_test_file(&TestType::OverwriteOnly, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                overwrite_file(&string_path)?;
                let bytes = get_bytes(&path)?;
                assert_eq!(bytes.len(), lorem.as_bytes().len());
                assert_ne!(bytes, lorem.as_bytes());
                std::fs::remove_file(&string_path).map_err(|_| {
                    Error::SystemProblem(FSProblem::Delete, string_path.to_string())
                })?;
                Ok(())
            }

            /// This test checks whether a 1KB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn small_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether a 1MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn medium_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!Path::new(&string_path).exists());
                Ok(())
            }

            /// This test checks whether a 10MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
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

            /// The test can be used to check whether a folder can be deleted using a particular method.
            ///
            /// Test success is all conditions are met :
            /// * a specific folder with multiple files in it is created
            /// * folder is delete thanks to the specific erasing method
            #[test]
            fn folder_test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::Folder, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether an error is returned when a file is read-only and a user tries to delete it using a particular method..
            ///
            /// Test success is all conditions are met :
            /// * A readonly file is created
            /// * An error is returned
            /// * The file is deleted at the end of the test
            #[test]
            fn permission_denied() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::WritingError, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                let result = EraseMethod.delete(&string_path);
                println!("{:?}", result);
                assert!(result.is_err());
                let mut perms = path.metadata().unwrap().permissions();
                perms.set_readonly(false);
                std::fs::set_permissions(&string_path, perms).map_err(|_| {
                    Error::SystemProblem(FSProblem::Permissions, string_path.clone())
                })?;
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        mod log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature log functions correctly for basic error handling.
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::LogMini, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(feature = "secure_log")]
        mod secure_log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature secure_log functions correctly for basic error handling.
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SecureLog, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }
    }

    /// Module containing all the tests for the error-stack handling method
    #[cfg(feature = "error-stack")]
    mod enhanced {
        use super::*;

        use crate::tests::enhanced::{create_test_file, get_bytes};
        use crate::{Error, Result};

        #[cfg(not(any(feature = "log", feature = "secure_log")))]
        mod no_log {
            use error_stack::ResultExt;
            use pretty_assertions::{assert_eq, assert_ne};
            use std::path::Path;

            use super::*;

            /// Test if the overwrite method for this particular erase protocol work well or not.
            ///
            /// Test success is all conditions are met :
            /// * function overwrite_file is success
            /// * file is overwritten
            /// * file is overwritten with good method
            /// * file is well deleted
            #[test]
            fn basic_overwrite() -> Result<()> {
                let (string_path, lorem) =
                    create_test_file(&TestType::OverwriteOnly, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                overwrite_file(&string_path)?;
                let bytes = get_bytes(&path)?;
                assert_eq!(bytes.len(), lorem.as_bytes().len());
                assert_ne!(bytes, lorem.as_bytes());
                std::fs::remove_file(&string_path).change_context(Error::SystemProblem(
                    FSProblem::Delete,
                    string_path.to_string(),
                ))?;
                Ok(())
            }

            /// This test checks whether a 1KB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn small_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SmallFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether a 1MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
            #[test]
            fn medium_deletion() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::MediumFile, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!Path::new(&string_path).exists());
                Ok(())
            }

            /// This test checks whether a 10MB file is correctly rewritten and deleted for a given delete method.
            ///
            /// Test success is all conditions are met :
            /// * a specific file is created
            /// * file is delete thanks to the specific erasing method
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

            /// The test can be used to check whether a folder can be deleted using a particular method.
            ///
            /// Test success is all conditions are met :
            /// * a specific folder with multiple files in it is created
            /// * folder is delete thanks to the specific erasing method
            #[test]
            fn folder_test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::Folder, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }

            /// This test checks whether an error is returned when a file is read-only and a user tries to delete it using a particular method..
            ///
            /// Test success is all conditions are met :
            /// * A readonly file is created
            /// * An error is returned
            /// * The file is deleted at the end of the test
            #[test]
            fn permission_denied() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::WritingError, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                let result = EraseMethod.delete(&string_path);
                println!("{:?}", result);
                assert!(result.is_err());
                let mut perms = path.metadata().unwrap().permissions();
                perms.set_readonly(false);
                std::fs::set_permissions(&string_path, perms).change_context(
                    Error::SystemProblem(FSProblem::Permissions, string_path.clone()),
                )?;
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(all(feature = "log", not(feature = "secure_log")))]
        mod log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature log functions correctly
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::LogMini, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }

        #[cfg(feature = "secure_log")]
        mod secure_log {
            use super::*;
            use std::path::Path;

            /// The test ensures that the feature secure_log functions correctly.
            ///
            /// Test success is all conditions are met :
            /// * A specific file is created
            /// * The file is deleted without any error
            #[test]
            fn test() -> Result<()> {
                let (string_path, _) = create_test_file(&TestType::SecureLog, &METHOD_NAME)?;
                let path = Path::new(&string_path);
                assert!(path.exists());
                EraseMethod.delete(&string_path)?;
                assert!(!path.exists());
                Ok(())
            }
        }
    }
}


```

## Update in [src/methods/mod.rs](src/methods/mod.rs)

### Add new method in enum
```rust
pub enum Method {
    /// DOD 522022 MECE erasing method <https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php>
    Dod522022MECE,
    ...
    #[default]
    PseudoRandom,
    // TODO add doc with link
    MethodName, // TODO : Update here
}
```

### Update Method display trait
```rust
// -- Region : Implement display trait for Method enum.
impl core::fmt::Display for Method {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Method::Dod522022MECE => write!(fmt, "DOD 522022 MECE"),
            ...
            Method::PseudoRandom => write!(fmt, "Pseudo Random"),
            // TODO Add new method display trait implementation
        }
    }
}

```
### Update delete file function
```rust
match self {
    Method::Dod522022MECE => dod_522022_me::overwrite_file(path)?.delete()?,
    ...
    Method::PseudoRandom => pseudo_random::overwrite_file(path)?.delete()?,
    // TODO : Add your method here
};

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