use std::fs;

mod abi;
mod instructions;
mod remix_json;
mod uint256;
mod vm;

use crate::abi::*;
use crate::remix_json::read_remix_json;
use crate::uint256::UInt256;
use crate::vm::*;

impl InputManager {
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

#[allow(dead_code)]
fn main() {
    // main_disassemble();

    let message = Message::new_call("get()");
    let filename = "bin/fixtures/Counter.bin";
    let contract = InputManager::from_file(&filename);
    match send_message_to_contract(message, contract) {
        Ok(()) => println!("DONE!"),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
