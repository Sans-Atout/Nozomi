# Changelog

## [v3.0.2](https://crates.io/crates/nozomi/3.0.2)

```diff
Project change :
! log : 0.4.22 --> 0.4.25
! rand : 0.8.5 --> 0.9.0
! pretty_assertions : 1.4.0 --> 1.4.1

+ Addition of a test to check whether the error enumeration respects the RFC-1236
! Move from std::error to core::error for better compatibility
```

## [v3.0.1](https://crates.io/crates/nozomi/3.0.1)

```diff
Project change :
! log : 0.4.21 --> 0.4.22
! error-stack : 0.4.1 --> 0.5.0

Documentation update :
! Correct typo in README file
```

## [v3.0.0](https://crates.io/crates/nozomi/3.0.0)
```diff
+ Add log and secure log features
+ Add more standard error handling logic as default error handling method
    reference : https://youtu.be/j-VQCYP7wyw

! Simplify the library API
! Making use of error-stack library an feature

! Changes in how tests are carried out
-   Remove test.sh file
!   Update the test logic following the deletion of the test.sh file
+   Add more precise tests
```
## [v2.0.3](https://crates.io/crates/nozomi/2.0.3)

```diff
Project change :
! log : 0.4.22 --> 0.4.25
! rand : 0.8.5 --> 0.9.0
```
## [v2.0.2](https://crates.io/crates/nozomi/2.0.2)

```diff
Project change :
!  Update error-stack from 0.4.1 to 0.5.0
```
## [v2.0.1](https://crates.io/crates/nozomi/2.0.1)
```diff
! Update dependencies
+ Add pretty_assertions as dev dependencies

Rerun all tests
```

## [v2.0.0](https://crates.io/crates/nozomi/2.0.0)
```diff
+ Apply ANSSI best practice for Rust
+ Apply error handling best practice
    reference can be found here : https://www.youtube.com/watch?v=g6WUHcyjsfc

! Refactoring code
+   Code easier to use for user
+   Code more flexible

+ Implement test
+ add test script 

! Update README file
+ CHANGELOG file
+ CONTRIBUTING file
+ CODE_OF_CONDUCT file
+ SECURITY file
+ examples folder
```

## [v1.0.2](https://crates.io/crates/nozomi/1.0.2)
```diff
! Update Readme file

- Correct a bug in erase_folder overwrite method
    When a folder is found in erase folder overwrite method, the overwrite method crash because a folder is not a file.

+ error handling
+ success handling
```

## [v1.0.1](https://crates.io/crates/nozomi/1.0.1)
```diff
! Update Readme file
```

## [v1.0.0](https://crates.io/crates/nozomi/1.0.0)

```diff
+ Pseudo Random overwrite method
+ Gutmann overwrite method
+ HMGI S5 overwrite method
+ DOD 522022 MECE overwrite method
+ DOD 522022 ME overwrite method
+ AFSSI 5020 overwrite method
+ RCMP TSSIT OPS II overwrite method

+ erase folder method
```