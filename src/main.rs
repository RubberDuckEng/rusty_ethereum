#![allow(dead_code)]

use itertools::Itertools;

// use bytes::Bytes;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fs;
use std::iter::Iterator;
use std::ops::Range;

mod instructions;
mod remix_json;
mod uint256;

use crate::instructions::*;
use crate::remix_json::*;
use crate::uint256::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum VMError {
    UNDERFLOW,
    BADOP(u8),
    BADARG,
    OUTOFBOUNDS,
}

impl fmt::Debug for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::UNDERFLOW => write!(f, "UNDERFLOW"),
            VMError::BADOP(op) => write!(f, "BADOP(0x{:02X})", op),
            VMError::BADARG => write!(f, "BADARG"),
            VMError::OUTOFBOUNDS => write!(f, "OUTOFBOUNDS"),
        }
    }
}

type Word = UInt256;

#[derive(Default)]
struct Stack {
    values: Vec<Word>,
}

impl Stack {
    fn push(&mut self, value: UInt256) {
        return self.values.push(value);
    }

    fn peek(&self, index: usize) -> Result<UInt256, VMError> {
        if index < self.values.len() {
            Ok(self.values[self.values.len() - index - 1])
        } else {
            Err(VMError::UNDERFLOW)
        }
    }

    fn pop(&mut self) -> Result<UInt256, VMError> {
        return self.values.pop().ok_or(VMError::UNDERFLOW);
    }
}

#[derive(Default)]
struct Memory {
    storage: Vec<u8>,
}

#[derive(Default)]
struct Message {
    value: UInt256, // message funds in wei
    caller: UInt256,
    data: Vec<UInt256>,
}

#[derive(Default)]
struct Task {
    message: Message, // This ends up being a stack I think?
    stack: Stack,
    memory: Memory,
    input: InputManager,
    return_data: Vec<u8>,
    is_complete: bool,
}

impl Task {
    fn new(input: InputManager, message: Message) -> Task {
        Task {
            input: input,
            message: message,
            ..Task::default()
        }
    }
}

fn to_usize_range(range: Range<UInt256>) -> Result<Range<usize>, VMError> {
    let start = usize::try_from(range.start).map_err(|_| VMError::OUTOFBOUNDS)?;
    let end = usize::try_from(range.end).map_err(|_| VMError::OUTOFBOUNDS)?;
    Ok(start..end)
}

impl Memory {
    fn ensure_size(&mut self, end: usize) {
        if end > self.storage.len() {
            self.storage.resize(end, 0);
        }
    }

    fn store(&mut self, offset: UInt256, value: UInt256) -> Result<(), VMError> {
        let index: usize = offset.try_into().map_err(|_| VMError::UNDERFLOW)?;
        let end = index + 32;
        self.ensure_size(end);
        value.to_be_bytes(&mut self.storage[index..end]);
        Ok(())
    }

    fn copy_out(&self, range: Range<UInt256>) -> Result<Vec<u8>, VMError> {
        let usize_range = to_usize_range(range)?;
        let mut out = vec![0u8; usize_range.end - usize_range.start];
        out.copy_from_slice(&self.storage[usize_range]);
        Ok(out)
    }
}

impl Task {
    fn execute_single_instruction(
        &mut self,
        instruction: &Instruction,
        arg_option: Option<UInt256>,
    ) -> Result<(), VMError> {
        assert!(!self.is_complete);
        let stack = &mut self.stack;
        print_instruction(instruction, arg_option);
        match *instruction {
            OP_ADD => {
                let result = stack.pop()? + stack.pop()?;
                stack.push(result);
            }
            OP_MSTORE => {
                let offset = stack.pop()?;
                let value = stack.pop()?;
                self.memory.store(offset, value)?;
            }
            OP_CALLVALUE => {
                stack.push(self.message.value);
            }
            OP_DUP1 => {
                stack.push(stack.peek(0)?);
            }
            OP_ISZERO => {
                if stack.peek(0)? == UInt256::ZERO {
                    stack.push(UInt256::ONE)
                } else {
                    stack.push(UInt256::ZERO)
                }
            }
            OP_CODECOPY => {
                let dest_offset = stack.pop()?;
                let offset = stack.pop()?;
                let length = stack.pop()?;
                // 	memory[destOffset:destOffset+length] = address(this).code[offset: offset + length]
                let from = to_usize_range(offset..offset + length)?;
                let to = to_usize_range(dest_offset..dest_offset + length)?;
                println!("{:?} {:?} {:?}", from, to, self.input.ops.len());
                // TODO: Does this index from input[0] or $PC or something else?
                self.memory.ensure_size(to.end);
                self.memory.storage[to].copy_from_slice(&self.input.ops[from]);
            }
            OP_JUMPI => {
                let destination = stack.pop()?;
                let condition = stack.pop()?;
                if condition == UInt256::ZERO {
                    self.input.index = destination.try_into().map_err(|_| VMError::UNDERFLOW)?;
                }
            }
            OP_JUMPDEST => {
                // Metadata to annotate possible jump destination, no action.
            }
            OP_POP => {
                stack.pop()?;
            }
            OP_RETURN => {
                let offset = stack.pop()?;
                let length = stack.pop()?;
                let range = offset..offset + length;
                self.return_data = self.memory.copy_out(range)?;
                self.is_complete = true;
            }
            OP_REVERT => {
                // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-140.md
                // TODO: Should revert all actions?
                let offset = stack.pop()?;
                let length = stack.pop()?;
                let range = offset..offset + length;
                self.return_data = self.memory.copy_out(range)?;
                self.is_complete = true;
            }
            // All push instructions:
            Instruction {
                op: 0x60..=0x7F, ..
            } => {
                let arg = arg_option.ok_or(VMError::BADARG)?;
                stack.push(arg);
            }
            _ => {
                return Err(VMError::BADOP(instruction.op));
            }
        }
        Ok(())
    }
    fn take_op(&mut self) -> Option<u8> {
        if self.is_complete {
            None
        } else {
            self.input.take_op()
        }
    }
    fn execute(&mut self) -> Result<(), VMError> {
        let ops: HashMap<_, _> = INSTRUCTIONS
            .iter()
            .map(|instruction| (instruction.op, instruction))
            .collect();
        while let Some(op) = self.take_op() {
            let inst = ops.get(&op).ok_or(VMError::BADOP(op))?;
            let arg_option = self.input.take_arg(inst.arg)?;
            self.execute_single_instruction(inst, arg_option)?;
        }
        Ok(())
    }
}

#[derive(Default)]
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

fn print_instruction(inst: &Instruction, arg_option: Option<UInt256>) {
    match arg_option {
        Some(arg) => println!("{:02X}: {} {}", inst.op, inst.name, arg),
        None => println!("{:02X}: {}", inst.op, inst.name),
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
        print_instruction(inst, arg_option);
    }

    Ok(())
}

fn main_disassemble() {
    // let filename = "bin/fixtures/Counter.bin";
    // println!("In file {}", filename);
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let filename = "fixtures/counter_bytecode_8_0_1_remix.json";
    let result = read_remix_json(filename);

    let mut input = InputManager::new(&result.object);

    match dissemble(&mut input) {
        Ok(()) => println!("DONE"),
        Err(error) => println!("ERROR: {:?}", error),
    }
}

fn main_execute() {
    let filename = "bin/fixtures/Counter.bin";
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input = InputManager::new(&contents);

    // let instructions: Vec<u8> = vec![OP_PUSH1.op, 1, OP_PUSH1.op, 2, OP_ADD.op];
    // let input = InputManager {
    //     ops: instructions,
    //     index: 0,
    // };

    let message = Message {
        value: UInt256::ONE, // Non-zero wei.
        caller: UInt256::ZERO,
        ..Message::default()
    };

    let mut task = Task::new(input, message);
    match task.execute() {
        Ok(()) => println!(
            "DONE: Stack: {:?}, Return Data: {:?}",
            task.stack.values, task.return_data
        ),
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
