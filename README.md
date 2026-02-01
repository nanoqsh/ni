<div align="center">
    <h1>ni</h1>
    <p>
        Small limited alloc-free named identifier
    </p>
    <p>
        <a href="https://crates.io/crates/ni"><img src="https://img.shields.io/crates/v/ni.svg"></img></a>
        <a href="https://docs.rs/ni"><img src="https://docs.rs/ni/badge.svg"></img></a>
    </p>
</div>

## Usage

This crate provides a type of small, limited, alloc-free string - `Name`, that can only store:

* Latin lowercase letters `'a'..='z'`  
* Digits `'0'..='9'`  
* Underscores `'_'`  

The string length cannot exceed 24. This makes it possible to store identifiers very efficiently, such as: `key12`, `hello_world`, `ni_ident_version2`, etc.

The `Name` type has the same size as `&str` - two pointers (16 bytes on a 64-bit platform). Unlike `&str`, it does not carry a lifetime because it stores the data internally. It also implements `Copy`, meaning it can be duplicated freely without cost since its size is tiny.

This type doesn't implement many string operations. Instead, it's mainly used for efficient in-memory storage.

## Example

Add the crate to your project. You can enable a feature to get serialization/deserialization support:

```sh
cargo add ni -F serde
```

Currently, the crate supports `serde` and `bytemuck`. This is optional if you only need to store many small strings in memory.

Now we can efficiently parse data with many identifiers, for example a schema with arbitrary fields like a `HashMap`:

```rust
use {
    ni::Name,
    std::{collections::HashMap, io},
};

fn main() -> io::Result<()> {
    let json = r#"
    {
        "some_data": 10,
        "key0": 1,
        "key1": 2,
        "key2": 3,
        "key3": 4
    }"#;

    let data: HashMap<Name, u32> = serde_json::from_str(&json)?;
    println!("{data:#?}");

    Ok(())
}
```
