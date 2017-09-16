[![Travis branch](https://img.shields.io/badge/rustc-nightly%20only-brightgreen.svg)](https://www.rust-lang.org/)
[![Latest Version](https://img.shields.io/crates/v/str_to_enum_derive.svg)](https://crates.io/crates/str_to_enum_derive)
[![Travis branch](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

# warning

Because of proc-macro, this lib is nightly only(1.22).

# example

`cargo.toml`

```toml
[dependencies]
str_to_enum_derive = "0.1.0"
```

`main.rs`

```rust
#[macro_use]
extern crate str_to_enum_derive;

use std::str::FromStr;

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