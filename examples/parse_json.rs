use {
    ni::Name,
    std::{collections::HashMap, fs, io},
};

fn main() -> io::Result<()> {
    let content = fs::read_to_string("examples/data.json")?;

    // If you get the error "the trait bound `Name: serde::Deserialize<'_>`
    // is not satisfied", enable the `serde` feature
    let data: HashMap<Name, u32> = serde_json::from_str(&content)?;
    println!("{data:#?}");

    Ok(())
}
