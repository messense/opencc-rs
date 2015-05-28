# opencc-rs

[![Build Status](https://travis-ci.org/messense/opencc-rs.svg)](https://travis-ci.org/messense/opencc-rs)
[![Coverage Status](https://coveralls.io/repos/messense/opencc-rs/badge.svg)](https://coveralls.io/r/messense/opencc-rs)
[![Crates.io](https://img.shields.io/crates/v/opencc.svg)](https://crates.io/crates/opencc)

OpenCC binding for Rust.

[Read the documentation](https://messense.github.io/opencc-rs).


## Installation

You should install [OpenCC 1.0.x](https://github.com/BYVoid/OpenCC) library first.

Add it to your ``Cargo.toml``:

```toml
[dependencies]
opencc = "*"
```

Add ``extern crate opencc`` to your crate root and your're good to go!
For example:

```rust
extern crate opencc;

use opencc::OpenCC;

fn main() {
    let cc = OpenCC::new("t2s.json");
    println!("{}", cc.convert("乾坤一擲"));
    println!("{}", cc.convert("開放中文轉換"));
}
```


## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
