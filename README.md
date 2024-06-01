# Nozomi

A Rust library that wipe all file or folder. \
This library includes most of the secure deletion methods of the [Eraser](https://eraser.heidi.ie) for Windows software.

# Add to your project
```toml
[dependencies]
nozomi = "3.0.0"
```

## Test libraries
### Prerequis (optional)

### Test
```shell
git clone https://github.com/Sans-Atout/Nozomi.git
cd Nozomi
cargo nextest run
```

## Code example
### Erase one file with Pseudo Random method
```rust
use nozomi;
use nozomi::OverwriteMethod::Afssi5020;

fn main() {
    match nozomi::erase_file("path_to_file", Afssi5020){
        Ok(info) => println!("{}",info),
        Err(error) => println!("{}",error)
    };
}
```

### Erase one folder with Pseudo Random method
```rust
use nozomi;
use nozomi::OverwriteMethod::Afssi5020;

fn main() {
    match nozomi::erase_folder("path_to_folder", Afssi5020, false){
        Ok(info) => println!("{}",info),
        Err(error) => println!("{}",error)
    };
}
```
# Support
A ce jour, trois versions de la librairie sont disponibles.

# [Changelog](CHANGELOG)
# [Contributing](CONTRIBUTING)

# [Erase Method](ERASE_METHOD.md)
