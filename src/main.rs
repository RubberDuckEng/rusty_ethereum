#![allow(dead_code)]

use itertools::Itertools;

// use bytes::Bytes;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::fs;
use std::iter::Iterator;

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

trait EndianRead {
    type Array;
    fn from_be_slice(bytes: &[u8]) -> Self;
}

macro_rules! impl_EndianRead_for_ints (( $($int:ident),* ) => {
    $(
        impl EndianRead for $int {
            type Array = [u8; std::mem::size_of::<Self>()];
            fn from_be_slice(bytes: &[u8]) -> Self {
                let array = bytes.try_into().unwrap();
                Self::from_be_bytes(array)
            }
        }
    )*
});

impl_EndianRead_for_ints!(u8, u16, u32, u64, u128);

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

    fn take<T: EndianRead>(&mut self) -> Result<T, VMError> {
        let size = std::mem::size_of::<T>();
        let end = self.index + size;
        if end <= self.ops.len() {
            let bytes = &self.ops[self.index..end];
            self.index += size;
            Ok(T::from_be_slice(bytes))
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
            ArgType::U8 => ArgValue::U8(self.take::<u8>()?),
            ArgType::U16 => ArgValue::U16(self.take::<u16>()?),
            ArgType::U32 => ArgValue::U32(self.take::<u32>()?),
            ArgType::U64 => ArgValue::U64(self.take::<u64>()?),
            ArgType::U128 => ArgValue::U128(self.take::<u128>()?),
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
        println!("{:02X}: {} {}", inst.op, inst.name, arg);
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
