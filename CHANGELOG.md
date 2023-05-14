# Changelog
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