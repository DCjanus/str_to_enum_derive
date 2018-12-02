[![Rust compiler level](https://img.shields.io/badge/edition-2018-brightgreen.svg)](https://www.rust-lang.org/)
[![Latest Version](https://img.shields.io/crates/v/str_to_enum_derive.svg)](https://crates.io/crates/str_to_enum_derive)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

# example

`cargo.toml`

```toml
[dependencies]
str_to_enum_derive = "0.2.0"
```

`main.rs`

```rust
#[macro_use]
extern crate str_to_enum_derive;

#[derive(StrToEnum, Debug, PartialEq, Eq)]
enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE
}

fn main() {
    assert_eq!(HTTPMethod::GET, "GET".parse::<HTTPMethod>().unwrap());
    assert_eq!(HTTPMethod::HEAD, "HEAD".parse::<HTTPMethod>().unwrap());
}
```

# TODO

+ [ ] attribute `value`
+ [ ] attribute `case` (with [heck](https://crates.io/crates/heck))
+ [ ] attribute `case_insensitive`

# Features

```toml
[dependencies]
str_to_enum_derive = { version = "*", features = ["fxhasher"] }
```

Replace [DefaultHasher](https://doc.rust-lang.org/std/collections/hash_map/struct.DefaultHasher.html) with [FxHasher](https://crates.io/crates/fxhash)

# Changelog

Maintained in [CHANGELOG.MD](./CHANGELOG.MD)