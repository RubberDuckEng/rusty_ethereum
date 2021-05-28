#![allow(dead_code)]

use itertools::Itertools;

// use bytes::Bytes;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::iter::Iterator;

use serde::Deserialize;
use serde_json;

mod instructions;
mod uint256;

use crate::instructions::*;
use crate::uint256::*;

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

type Word = UInt256;

#[derive(Default)]
struct Stack {
    pub values: Vec<Word>,
}

fn execute(
    stack: &mut Stack,
    instruction: &Instruction,
    arg_option: Option<UInt256>,
) -> Result<(), VMError> {
    match *instruction {
        OP_ADD => {
            let a = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            let b = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            stack.values.push(a + b);
        }
        OP_PUSH1 => {
            let word = arg_option.ok_or(VMError::BADARG)?;
            stack.values.push(word);
        }
        _ => {
            return Err(VMError::BADOP(instruction.op));
        }
    }
    Ok(())
}

fn playground(stack: &mut Stack) -> Result<(), VMError> {
    let ops: HashMap<_, _> = INSTRUCTIONS
        .iter()
        .map(|instruction| (instruction.op, instruction))
        .collect();

    let instructions: Vec<u8> = vec![OP_PUSH1.op, 1, OP_PUSH1.op, 2, OP_ADD.op];
    let mut input = InputManager {
        ops: instructions,
        index: 0,
    };
    while let Some(op) = input.take_op() {
        let inst = ops.get(&op).ok_or(VMError::BADOP(op))?;
        let arg_option = input.take_arg(inst.arg)?;
        execute(stack, inst, arg_option)?;
    }

    Ok(())
}

struct InputManager {
    ops: Vec<u8>,
    index: usize,
}

impl ArgType {
    pub fn size(&self) -> Option<usize> {
        match self {
            ArgType::Void => None,
            ArgType::U(bits) => Some(bits / 8),
        }
    }
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
        arg_type.size().map(|size| self.take(size)).transpose()
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

#[derive(Deserialize)]
struct RemixCompileResult {
    object: String,
    opcodes: String,
}

fn main_disassemble() {
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
}

fn main_execute() {
    let mut stack = Stack::default();
    match playground(&mut stack) {
        Ok(()) => println!("DONE: {:?}", stack.values),
        Err(error) => println!("ERROR: {:?}", error),
    }
}

fn main() {
    // main_disassemble();
    main_execute();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
