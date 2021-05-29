use itertools::Itertools;

use std::fs;
use std::iter::Iterator;

mod abi;
mod instructions;
mod remix_json;
mod uint256;
mod vm;

use crate::abi::Message;
use crate::remix_json::read_remix_json;
use crate::uint256::UInt256;
use crate::vm::*;

impl InputManager {
    fn from_string(contents: &String) -> InputManager {
        let chars = contents.chars();
        let chunks = chars.chunks(2);
        let ops = chunks
            .into_iter()
            .map(|chunk| u8::from_str_radix(&chunk.collect::<String>(), 16).expect("vaild hex"))
            .collect::<Vec<u8>>();
        InputManager::from_bytes(ops)
    }

    fn from_file(filename: &str) -> InputManager {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        InputManager::from_string(&contents)
    }
}

#[allow(dead_code)]
fn main_disassemble() {
    // let filename = "bin/fixtures/Counter.bin";
    // let mut input = InputManager::from_file(filename);

    let filename = "fixtures/counter_bytecode_8_0_1_remix.json";
    let result = read_remix_json(filename);
    let mut input = InputManager::from_string(&result.object);

    match dissemble(&mut input) {
        Ok(()) => println!("DONE"),
        Err(error) => println!("ERROR: {:?}", error),
    }
}

fn main() {
    // main_disassemble();

    // Method function names:
    // https://docs.soliditylang.org/en/develop/abi-spec.html
    // sha256("name(args..,returns)")
    let message = Message::new_call("get(uint256)");
    let filename = "bin/fixtures/Counter.bin";
    let contract = InputManager::from_file(&filename);
    match send_message_to_contract(message, contract) {
        Ok(()) => println!("DONE!"),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
