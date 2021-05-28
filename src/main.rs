#![allow(dead_code)]

use itertools::Itertools;

// use bytes::Bytes;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::iter::Iterator;

mod instructions;

use crate::instructions::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum VMError {
    UNDERFLOW,
    BADOP(u8),
    BADARG,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Word(u32);

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

// #[derive(Default)]
// struct Parser {
//     pub input: Bytes,
// }

// impl Parser {
//     fn read_op_code(&self) -> Instruction::new(}
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

    fn take_u16(&mut self) -> Result<u16, VMError> {
        let size = std::mem::size_of::<u16>();
        let end = self.index + size;
        if end <= self.ops.len() {
            let bytes = &self.ops[self.index..end];
            self.index += size;
            Ok(u16::from_be_bytes(
                bytes.try_into().map_err(|_| VMError::BADARG)?,
            ))
        } else {
            Err(VMError::BADARG)
        }
    }

    fn take_op(&mut self) -> Option<u8> {
        self.take_u8().ok()
    }

    fn take_arg(&mut self, arg_type: ArgType) -> Result<ArgValue, VMError> {
        Ok(match arg_type {
            ArgType::Void => ArgValue::Void,
            ArgType::U8 => ArgValue::U8(self.take_u8()?),
            ArgType::U16 => ArgValue::U16(self.take_u16()?),
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
        let arg = input.take_arg(inst.arg)?;
        println!("{:02x}: {} {}", inst.op, inst.name, arg);
        // 00  NAME
        // 01  ??
        // 02  PUSH1 ARG

        // match op {
        //     OP_ADD => println!("{:02x}: ADD", op),
        //     OP_PUSH1 => {
        //         let arg = take().unwrap(); // TODO: Handle overflow.
        //         println!("{:02x}: PUSH1 ({:02x})", op, arg);
        //     }
        //     _ => println!("{:02x}: ??", op),
        // }
    }

    Ok(())
}

fn main() {
    let filename = "bin/fixtures/Counter.bin";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut input = InputManager::new(&contents);

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

    println!("With text:\n{}", contents);

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
