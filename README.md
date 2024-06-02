# Nozomi

A Rust library that wipe all file or folder. \
This library includes most of the secure deletion methods of the [Eraser](https://eraser.heidi.ie) for Windows software.

# Add to your project
```toml
[dependencies]
nozomi = "3.0.0"
```

## Test libraries
### Prerequire (optional)
```shell
cargo install cargo-nextest
```

### Test
```shell
git clone https://github.com/Sans-Atout/Nozomi.git
cd Nozomi
cargo nextest run
```

## Code example
```rust
use nozomi::Method::Afssi5020;

fn main() -> Result<(),nozomi::Error> {
    Afssi5020::delete("path/to/file.txt")?;
    // OR
    match Afssi5020::delete("path/to/file.txt"){
        Ok(info) => println!("{}",info), // If you want
        Err(error) => println!("{}",error)
    };

    Ok(())
}
```

# Support
## End of life dates 
| Version | Support | End of phase (dd/mm/aaaa) |
|--|--|--|
|3.x|Supported|  |
|2.x|Passively supported|02-06-2029|
|1.x|End of life process|02-06-2025|

## Support life cycle
When a new major version (N) is released, it will become actively supported. Bugs will be fixed and new features will be added (new default deletion algorithm, better documentation, etc.).
The library will be audited every week with the `cargo audit` command to ensure that no flaws persist in the solution.

The previous major version (N-1) will enter in the passive support phase, which will last 5 years. 
During this period, the library dependencies will be updated every three months to ensure that the project is running as up-to-date as possible. The code will also be audited, but only on a monthly basis with the `cargo audit`. If a CVE requiring a modification to the library code is discovered, a new minor version will be published. 

Once this passive support phase is over, the version will enter in the end-of-life process, which will last 1 year. During this phase, no more dependencies will be updated and no more issues concerning this library will be taken into account. This phase exists to give projects that may use the library additional time to make the necessary changes to their code before moving the version to ‘Yanked’ on [crates.io](https://crates.io/crates/nozomi/versions)

# Features
| Features |Explanation   |
|--|--|
| error-stack | allows the use of the error-stack library for error handling instead of the standard Rust error handling |
| log | Allows logs to be used within the library. However, as these logs allow the name of the deleted file / folder to be recovered. |
| secure_log | Allows you to display logs giving an idea of the progress of the rewriting functions but keeping the overwritten file/folder ‘secret’ by using the md5 hash algorithm. |

# [Changelog](CHANGELOG)
# [Contributing](CONTRIBUTING)

# [Erase Method](ERASE_METHOD.md)
