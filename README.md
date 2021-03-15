# ntext-rs

[![Documentation](https://github.com/uttarayan21/ntext-rs/actions/workflows/docs.yaml/badge.svg)](https://github.com/uttarayan21/ntext-rs/actions/workflows/docs.yaml)
[![Rust Build Test](https://github.com/uttarayan21/ntext-rs/actions/workflows/rust.yaml/badge.svg)](https://github.com/uttarayan21/ntext-rs/actions/workflows/rust.yaml)

Documentation of [ntext-rs](https://uttarayan21.github.io/ntext-rs) generated by cargo doc.

A rust library to get numbers (usize) as words

`12345 ` -> `Twelve Thousand,Three Hundred,Forty-Five`

Add to cargo.toml

```toml
[dependencies]
ntext = { git = "https://github.com/uttarayan21/ntext" }
```

Example program

```rust
extern crate ntext;
use ntext::to_text;
fn main() {
    println!("{}",to_text!(12345));
}
```

which should output  
`Twelve Thousand,Three Hundred,Forty-Five`

Check the [documentation](https://uttarayan21.github.io/ntext-rs) which has more examples and is usually up to date.
