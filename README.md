# Nozomi

A Rust librairy that wipe all file or folder. \
This library includes most of the secure deletion methods of the [Eraser](https://eraser.heidi.ie) for Windows software.

# Add to your project
```
[dependencies]
nozomi = "1.0.2"
```

## Code example
### Erase one file with Pseudo Random method
```rust
use nozomi;

fn main() {
    match nozomi::erase_file("path_to_file", nozomi::EraserEntity::PseudoRandom){
        Ok(info) => println!("{}",info),
        Err(error) => println!("{}",error)
    };
}
```

### Erase one folder with Pseudo Random method
```rust
use nozomi;

fn main() {
    match nozomi::erase_folder("path_to_folder", nozomi::EraserEntity::PseudoRandom){
        Ok(info) => println!("{}",info),
        Err(error) => println!("{}",error)
    };
}
```
# Changelog
## [v1.0.2](https://crates.io/crates/nozomi/1.0.2)
```
[+] Update Readme file
[B] Correct a bug in erase_folder function
    [C] When a folder is found in erase folder function, the function crash
        because a folder is not a file
[+] Adding error handling
[+] Adding success handling
```

## [v1.0.1](https://crates.io/crates/nozomi/1.0.1)
```
[+] Update Readme file
```

## [v1.0.0](https://crates.io/crates/nozomi/1.0.0)
```
[+] Adding erase file method
    [+] Pseudo Random
    [+] Gutmann
    [+] HMGI S5
    [+] DOD 522022 MECE
    [+] DOD 522022 ME
    [+] AFSSI 5020
    [+] RCMP TSSIT OPS II

[+] Adding erase folder method
```

# Erase Method
## [Pseudo Random](https://www.lifewire.com/data-sanitization-methods-2626133#toc-random-data)

| Pass number | Patern |
|------------:|-------:|
|1|# Random|

## [Gutmann](https://en.wikipedia.org/wiki/Gutmann_method)

| Pass number | Patern |
|------------:|-------:|
|1|# Random|
|2|# Random|
|3|# Random|
|4|# Random|
|5|0x55 0x55 0x55|
|6|0xAA 0xAA 0xAA|
|7|0x92 0x49 0x24|
|8|0x49 0x24 0x92|
|9|0x24 0x92 0x49|
|10|0x00 0x00 0x00|
|11|0x11 0x11 0x11|
|12|0x22 0x22 0x22|
|13|0x33 0x33 0x33|
|14|0x44 0x44 0x44|
|15|0x55 0x55 0x55|
|16|0x66 0x66 0x66|
|17|0x77 0x77 0x77|
|18|0x88 0x88 0x88|
|19|0x99 0x99 0x99|
|20|0xAA 0xAA 0xAA|
|21|0xBB 0xBB 0xBB|
|22|0xCC 0xCC 0xCC|
|23|0xDD 0xDD 0xDD|
|24|0xEE 0xEE 0xEE|
|25|0xFF 0xFF 0xFF|
|26|0x92 0x49 0x24|
|27|0x49 0x24 0x92|
|28|0x24 0x92 0x49|
|29|0x6D 0xB6 0xDB|
|30|0xB6 0xDB 0x6D|
|31|0xDB 0x6D 0xB6|
|32|# Random|
|33|# Random|
|34|# Random|
|35|# Random|

## [Hmgi S5](https://www.bitraser.com/knowledge-series/data-destruction-standards-and-guidelines.php)
| Pass number | Patern |
|------------:|-------:|
|1|0x00|
|2|# Random|

## [DOD 522022 MECE](https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php)

| Pass number | Patern |
|------------:|-------:|
|1|0x00|
|2|0xFF|
|3|# Random | 
|4|0x00|
|5|0x00|
|6|0xFF|
|7|# Random | 


## [DOD 522022 ME](https://www.bitraser.com/article/DoD-5220-22-m-standard-for-drive-erasure.php)
| Pass number | Patern |
|------------:|-------:|
|1|0x00|
|2|0xFF|
|3|# Random |

## [AFSSI 5020](https://www.lifewire.com/data-sanitization-methods-2626133#toc-afssi-5020)
| Pass number | Patern |
|------------:|-------:|
|1|0x00|
|2|0xFF|
|3|# Random |

## [RCMP TSSIT OPS II](https://www.datadestroyers.eu/technology/rcmp_tssit_ops-2.html)
| Pass number | Patern |
|------------:|-------:|
|1|0x00|
|2|0xFF|
|3|0x00|
|4|0xFF|
|5|0x00|
|6|0xFF|
|7|# Random |