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

This crate provides a type of small, limited, alloc-free string that can only store:

* Latin lowercase letters `'a'..='z'`  
* Digits `'0'..='9'`  
* Underscores `'_'`  

The string length cannot exceed 24. This makes it possible to store identifiers very efficiently, such as: `key12`, `hello_world`, `ni_ident_version2`, etc.
