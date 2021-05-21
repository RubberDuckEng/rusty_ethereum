#![allow(dead_code)]

use itertools::Itertools;

// use bytes::Bytes;
use std::fs;
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

const OP_STOP: Instruction = 0x00;
const OP_ADD: Instruction = 0x01;
const OP_MUL: Instruction = 0x02;
const OP_SUB: Instruction = 0x03;
const OP_DIV: Instruction = 0x04;
const OP_SDIV: Instruction = 0x05;
const OP_MOD: Instruction = 0x06;
const OP_SMOD: Instruction = 0x07;
const OP_ADDMOD: Instruction = 0x08;
const OP_MULMOD: Instruction = 0x09;
const OP_EXP: Instruction = 0x0a;
const OP_SIGNEXTEND: Instruction = 0x0b;
const OP_LT: Instruction = 0x10;
const OP_GT: Instruction = 0x11;
const OP_SLT: Instruction = 0x12;
const OP_SGT: Instruction = 0x13;
const OP_EQ: Instruction = 0x14;
const OP_ISZERO: Instruction = 0x15;
const OP_AND: Instruction = 0x16;
const OP_OR: Instruction = 0x17;
const OP_XOR: Instruction = 0x18;
const OP_NOT: Instruction = 0x19;
const OP_BYTE: Instruction = 0x1a;
const OP_SHA3: Instruction = 0x20;
const OP_ADDRESS: Instruction = 0x30;
const OP_BALANCE: Instruction = 0x31;
const OP_ORIGIN: Instruction = 0x32;
const OP_CALLER: Instruction = 0x33;
const OP_CALLVALUE: Instruction = 0x34;
const OP_CALLDATALOAD: Instruction = 0x35;
const OP_CALLDATASIZE: Instruction = 0x36;
const OP_CALLDATACOPY: Instruction = 0x37;
const OP_CODESIZE: Instruction = 0x38;
const OP_CODECOPY: Instruction = 0x39;
const OP_GASPRICE: Instruction = 0x3a;
const OP_EXTCODESIZE: Instruction = 0x3b;
const OP_EXTCODECOPY: Instruction = 0x3c;
const OP_BLOCKHASH: Instruction = 0x40;
const OP_COINBASE: Instruction = 0x41;
const OP_TIMESTAMP: Instruction = 0x42;
const OP_NUMBER: Instruction = 0x43;
const OP_DIFFICULTY: Instruction = 0x44;
const OP_GASLIMIT: Instruction = 0x45;
const OP_POP: Instruction = 0x50;
const OP_MLOAD: Instruction = 0x51;
const OP_MSTORE: Instruction = 0x52;
const OP_MSTORE8: Instruction = 0x53;
const OP_SLOAD: Instruction = 0x54;
const OP_SSTORE: Instruction = 0x55;
const OP_JUMP: Instruction = 0x56;
const OP_JUMPI: Instruction = 0x57;
const OP_PC: Instruction = 0x58;
const OP_MSIZE: Instruction = 0x59;
const OP_GAS: Instruction = 0x5a;
const OP_JUMPDEST: Instruction = 0x5b;
const OP_PUSH1: Instruction = 0x60;
// 0x60 -- 0x7f	PUSH*
// 0x80 -- 0x8f	DUP*
// 0x90 -- 0x9f	SWAP*
const OP_LOG0: Instruction = 0xa0;
const OP_LOG1: Instruction = 0xa1;
const OP_LOG2: Instruction = 0xa2;
const OP_LOG3: Instruction = 0xa3;
const OP_LOG4: Instruction = 0xa4;
const OP_CREATE: Instruction = 0xf0;
const OP_CALL: Instruction = 0xf1;
const OP_CALLCODE: Instruction = 0xf2;
const OP_RETURN: Instruction = 0xf3;
const OP_DELEGATECALL: Instruction = 0xf4;
const OP_INVALID: Instruction = 0xfe;
const OP_SELFDESTRUCT: Instruction = 0xff;

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

// #[derive(Default)]
// struct Parser {
//     pub input: Bytes,
// }

// impl Parser {
//     fn read_op_code(&self) -> Instruction {}
// }

fn main() {
    let filename = "bin/fixtures/Counter.bin";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let chars = contents.chars();

    // Iterate through the sring
    // Printing out pairs of chars as hex and decimal

    let chunks = chars.chunks(2);
    let ops = chunks
        .into_iter()
        .map(|chunk| u8::from_str_radix(&chunk.collect::<String>(), 16).expect("vaild hex"))
        .collect::<Vec<u8>>();

    let mut i = 0;
    let mut take = || {
        if i < ops.len() {
            let val = ops[i];
            i += 1;
            Some(val)
        } else {
            None
        }
    };

    while let Some(op) = take() {
        // 00  NAME
        // 01  ??
        // 02  NAME (ARG, ARG)

        match op {
            OP_ADD => println!("{:02x}: ADD", op),
            OP_PUSH1 => {
                let arg = take().unwrap(); // TODO: Handle overflow.
                println!("{:02x}: PUSH1 ({:02x})", op, arg);
            }
            _ => println!("{:02x}: ??", op),
        }
    }

    // while let code = chars.take(2) {
    // }\

    // chars.take(2);
    // Read the string
    // Convert from hex to binary
    // Read fixed-size

    // let mut parser = { contents.bytes() };

    println!("With text:\n{}", contents);

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
