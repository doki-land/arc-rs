use arc_ast::{Value, AST};
use arc_rs::{ParserConfig, Result};
use std::{
    fs::{self, read_to_string},
    path::Path,
};


mod json_compatibility;
mod easy_structure;
mod hard_structure;
mod display;

fn parse(file: impl AsRef<Path>) -> Result<AST> {
    let parser = ParserConfig::default();
    let ast = parser.parse(&read_to_string(file.as_ref())?)?;
    Ok(ast)
}

#[test]
fn ready() {
    println!("it, works!")
}
