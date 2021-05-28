#![allow(dead_code)]

use itertools::Itertools;

// use bytes::Bytes;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::iter::Iterator;

use serde::{Deserialize, Serialize};
use serde_json;

mod instructions;

use crate::instructions::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum VMError {
    UNDERFLOW,
    BADOP(u8),
    BADARG,
}

impl fmt::Debug for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::UNDERFLOW => write!(f, "UNDERFLOW"),
            VMError::BADOP(op) => write!(f, "BADOP(0x{:02X})", op),
            VMError::BADARG => write!(f, "BADARG"),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct UInt256 {
    high: u128,
    low: u128,
}

fn u128_from_be_slice(bytes: &[u8]) -> u128 {
    let mut word: u128 = 0;
    for byte in bytes {
        word <<= 8;
        word += *byte as u128;
    }
    return word;
}

impl UInt256 {
    pub fn from_be_slice(bytes: &[u8]) -> UInt256 {
        if bytes.len() > 8 {
            return UInt256 {
                high: u128_from_be_slice(&bytes[8..]),
                low: u128_from_be_slice(&bytes[..8]),
            };
        }
        return UInt256 {
            high: 0,
            low: u128_from_be_slice(bytes),
        };
    }
}

impl fmt::Display for UInt256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.high == 0 {
            write!(f, "(0x{:02X})", self.low)
        } else {
            write!(f, "(0x{:X}{:02X})", self.high, self.low)
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Word(UInt256);

#[derive(Default)]
struct Stack {
    pub values: Vec<Word>,
}

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// enum Instruction::new(
//     Add(),
//     Push1(u8),
// }

// struct InstructionStream {
//     ip: usize,
// }

// fn execute(
//     stack: &mut Stack,
//     op: Instruction,
//     more: &mut std::slice::Iter<Instruction>,
// ) -> Result<(), VMError> {
//     println!("{:?}", op);
//     match op {
//         OP_ADD => {
//             let a = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
//             let b = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
//             stack.values.push(Word(a.0 + b.0));
//         }
//         OP_PUSH1 => stack
//             .values
//             .push(Word(*more.next().ok_or(VMError::BADOP)? as u32)),
//         _ => {
//             return Err(VMError::BADOP);
//         }
//     }
//     Ok(())
// }

// fn playground(stack: &mut Stack) -> Result<(), VMError> {
//     let instructions: Vec<Instruction> = vec![OP_PUSH1, 1, OP_PUSH1, 2, OP_ADD];
//     let mut iter = instructions.iter();
//     while let Some(op) = iter.next() {
//         execute(stack, *op, &mut iter)?;
//     }

//     Ok(())
// }

struct InputManager {
    ops: Vec<u8>,
    index: usize,
}

impl InputManager {
    fn new(contents: &String) -> InputManager {
        let chars = contents.chars();
        let chunks = chars.chunks(2);
        let ops = chunks
            .into_iter()
            .map(|chunk| u8::from_str_radix(&chunk.collect::<String>(), 16).expect("vaild hex"))
            .collect::<Vec<u8>>();
        InputManager { ops, index: 0 }
    }

    fn take_u8(&mut self) -> Result<u8, VMError> {
        if self.index < self.ops.len() {
            let value = self.ops[self.index];
            self.index += 1;
            Ok(value)
        } else {
            Err(VMError::BADARG)
        }
    }

    fn take(&mut self, size: usize) -> Result<UInt256, VMError> {
        let end = self.index + size;
        if end <= self.ops.len() {
            let bytes = &self.ops[self.index..end];
            self.index += size;
            Ok(UInt256::from_be_slice(bytes))
        } else {
            Err(VMError::BADARG)
        }
    }

    fn take_op(&mut self) -> Option<u8> {
        self.take_u8().ok()
    }

    fn take_arg(&mut self, arg_type: ArgType) -> Result<Option<UInt256>, VMError> {
        Ok(match arg_type {
            ArgType::Void => None,
            ArgType::U8 => Some(self.take(1)?),
            ArgType::U16 => Some(self.take(2)?),
            ArgType::U24 => Some(self.take(3)?),
            ArgType::U32 => Some(self.take(4)?),
            ArgType::U40 => Some(self.take(5)?),
            ArgType::U48 => Some(self.take(6)?),
            ArgType::U56 => Some(self.take(7)?),
            ArgType::U64 => Some(self.take(8)?),
            ArgType::U72 => Some(self.take(9)?),
            ArgType::U80 => Some(self.take(10)?),
            ArgType::U88 => Some(self.take(11)?),
            ArgType::U96 => Some(self.take(12)?),
            ArgType::U104 => Some(self.take(13)?),
            ArgType::U112 => Some(self.take(14)?),
            ArgType::U120 => Some(self.take(15)?),
            ArgType::U128 => Some(self.take(16)?),
            ArgType::U136 => Some(self.take(17)?),
            ArgType::U144 => Some(self.take(18)?),
            ArgType::U152 => Some(self.take(19)?),
            ArgType::U160 => Some(self.take(20)?),
            ArgType::U168 => Some(self.take(21)?),
            ArgType::U176 => Some(self.take(22)?),
            ArgType::U184 => Some(self.take(23)?),
            ArgType::U192 => Some(self.take(24)?),
            ArgType::U200 => Some(self.take(25)?),
            ArgType::U208 => Some(self.take(26)?),
            ArgType::U216 => Some(self.take(27)?),
            ArgType::U224 => Some(self.take(28)?),
            ArgType::U232 => Some(self.take(29)?),
            ArgType::U240 => Some(self.take(30)?),
            ArgType::U248 => Some(self.take(31)?),
            ArgType::U256 => Some(self.take(32)?),
        })
    }
}

fn dissemble(input: &mut InputManager) -> Result<(), VMError> {
    let ops: HashMap<_, _> = INSTRUCTIONS
        .iter()
        .map(|instruction| (instruction.op, instruction))
        .collect();

    while let Some(op) = input.take_op() {
        let inst = ops.get(&op).ok_or(VMError::BADOP(op))?;
        let arg_option = input.take_arg(inst.arg)?;
        match arg_option {
            Some(arg) => println!("{:02X}: {} {}", inst.op, inst.name, arg),
            None => println!("{:02X}: {}", inst.op, inst.name),
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct RemixCompileResult {
    object: String,
    opcodes: String,
}

fn main() {
    // let filename = "bin/fixtures/Counter.bin";
    // println!("In file {}", filename);
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let filename = "fixtures/counter_bytecode_8_0_1_remix.json";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let compile_result: RemixCompileResult = serde_json::from_str(&contents).unwrap();

    let mut input = InputManager::new(&compile_result.object);

    match dissemble(&mut input) {
        Ok(()) => println!("DONE"),
        Err(error) => println!("ERROR: {:?}", error),
    }

    // while let code = chars.take(2) {
    // }\

    // chars.take(2);
    // Read the string
    // Convert from hex to binary
    // Read fixed-size

    // let mut parser = { contents.bytes() };

    // println!("With text:\n{}", contents);

    // let mut stack = Stack::default();
    // match playground(&mut stack) {
    //     Ok(()) => println!("DONE: {:?}", stack.values),
    //     Err(error) => println!("ERROR: {:?}", error),
    // }
}

// MVP
// Take an instruction input buffer
// pop in a loop
// Execute

// Add 0x1
// Push1 0x60

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
