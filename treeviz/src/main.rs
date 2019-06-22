use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct SkillTree {
    group: Vec<Group>
}

#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    application: Option<String>,
    requires: Option<Vec<String>>,
    items: Vec<Item>
}

#[derive(Debug, Deserialize)]
struct Item {
    name: String,
}

#[paw::main]
fn main(args: paw::Args) -> Result<(), Box<std::error::Error>> {
    for arg in args.skip(1) {
        let path = PathBuf::from(arg);
        println!("{:?}", path);
        let text = std::fs::read_to_string(&path)?;
        let decoded: SkillTree = toml::from_str(&text)?;
        println!("{:#?}", decoded);
    }
    Ok(())
}
