#![allow(dead_code)]

use std::iter::Iterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum VMError {
    UNDERFLOW,
    BADOP,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Word(u32);

#[derive(Default)]
struct Stack {
    pub values: Vec<Word>,
}

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// enum Instruction {
//     Add(),
//     Push1(u8),
// }

type Instruction = u8;

const OP_ADD: Instruction = 0x01;
const OP_PUSH1: Instruction = 0x02;

// struct InstructionStream {
//     ip: usize,
// }

fn execute(
    stack: &mut Stack,
    op: Instruction,
    more: &mut std::slice::Iter<Instruction>,
) -> Result<(), VMError> {
    println!("{:?}", op);
    match op {
        OP_ADD => {
            let a = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            let b = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            stack.values.push(Word(a.0 + b.0));
        }
        OP_PUSH1 => stack
            .values
            .push(Word(*more.next().ok_or(VMError::BADOP)? as u32)),
        _ => {
            return Err(VMError::BADOP);
        }
    }
    Ok(())
}

fn playground(stack: &mut Stack) -> Result<(), VMError> {
    let instructions: Vec<Instruction> = vec![OP_PUSH1, 1, OP_PUSH1, 2, OP_ADD];
    let mut iter = instructions.iter();
    while let Some(op) = iter.next() {
        execute(stack, *op, &mut iter)?;
    }

    Ok(())
}

fn main() {
    let mut stack = Stack::default();
    match playground(&mut stack) {
        Ok(()) => println!("DONE: {:?}", stack.values),
        Err(error) => println!("ERROR: {:?}", error),
    }
}

// MVP
// Take an instruction input buffer
// pop in a loop
// Execute

// Add 0x1
// Push1 0x60
