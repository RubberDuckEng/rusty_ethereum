use std::fs;

use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct RemixCompileResult {
    pub object: String,
    pub opcodes: String,
}

pub fn read_remix_json(filename: &str) -> RemixCompileResult {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return serde_json::from_str(&contents).unwrap();
}
