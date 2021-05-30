use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::iter::Iterator;
use std::ops::Range;

use crate::instructions::*;
use crate::uint256::*;
use crate::Message;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VMError {
    StackUnderflow,
    EndOfInstructions,
    BadAccess,
    BadOp(u8),
    BadArg,
    OutOfBounds,
    TypeConversion,
}

impl fmt::Debug for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::StackUnderflow => write!(f, "STACK_UNDERFLOW"),
            VMError::BadOp(op) => write!(f, "BadOp(0x{:02X})", op),
            VMError::BadArg => write!(f, "BadArg"),
            VMError::BadAccess => write!(f, "BadAccess"),
            VMError::OutOfBounds => write!(f, "OutOfBounds"),
            VMError::EndOfInstructions => write!(f, "END_OF_INSTRUCTIONS"),
            VMError::TypeConversion => write!(f, "TypeConversion"),
        }
    }
}

type Word = UInt256;

#[derive(Default)]
struct Stack {
    values: Vec<Word>,
}

impl Stack {
    fn from_top(&self, index: usize) -> Result<usize, VMError> {
        if index < self.values.len() {
            Ok(self.values.len() - index - 1)
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    fn push(&mut self, value: UInt256) {
        return self.values.push(value);
    }

    fn peek(&self, index: usize) -> Result<UInt256, VMError> {
        Ok(self.values[self.from_top(index)?])
    }

    fn pop(&mut self) -> Result<UInt256, VMError> {
        return self.values.pop().ok_or(VMError::StackUnderflow);
    }

    fn swap(&mut self, a: usize, b: usize) -> Result<(), VMError> {
        let a_usize = self.from_top(a)?;
        let b_usize = self.from_top(b)?;
        Ok(self.values.swap(a_usize, b_usize))
    }
}

#[derive(Default)]
struct Memory {
    bytes: Vec<u8>,
}

#[derive(Default)]
struct Storage {}

impl Storage {
    fn load(&self, key: UInt256) -> UInt256 {
        println!("Storage.load({}) not implemented, returning 0", key);
        UInt256::ZERO
    }
}

pub struct Task<'a> {
    message: &'a Message, // This ends up being a stack I think?
    stack: Stack,
    memory: Memory,
    input: InputManager,
    storage: Storage,
}

impl Task<'_> {
    fn new(input: InputManager, message: &Message) -> Task {
        Task {
            input: input,
            message: message,
            stack: Stack::default(),
            memory: Memory::default(),
            storage: Storage::default(),
        }
    }
}

fn to_usize_range(range: Range<UInt256>) -> Result<Range<usize>, VMError> {
    let start = usize::try_from(range.start).map_err(|_| VMError::OutOfBounds)?;
    let end = usize::try_from(range.end).map_err(|_| VMError::OutOfBounds)?;
    Ok(start..end)
}

impl Memory {
    fn ensure_size(&mut self, end: usize) {
        if end > self.bytes.len() {
            self.bytes.resize(end, 0);
        }
    }
    fn load(&self, offset: UInt256) -> Result<UInt256, VMError> {
        let index: usize = offset.try_into().map_err(|_| VMError::BadAccess)?;
        let end = index + 32;
        Ok(UInt256::from_be_slice(&self.bytes[index..end]))
    }

    fn store(&mut self, offset: UInt256, value: UInt256) -> Result<(), VMError> {
        let index: usize = offset.try_into().map_err(|_| VMError::BadAccess)?;
        let end = index + 32;
        self.ensure_size(end);
        value.to_be_bytes(&mut self.bytes[index..end]);
        Ok(())
    }

    fn copy_out(&self, range: Range<UInt256>) -> Result<Vec<u8>, VMError> {
        let usize_range = to_usize_range(range)?;
        let mut out = vec![0u8; usize_range.end - usize_range.start];
        out.copy_from_slice(&self.bytes[usize_range]);
        Ok(out)
    }
}

enum InstructionResult {
    Continue,
    Return(Vec<u8>),
    Revert(Vec<u8>),
}

enum TaskResult {
    Return(Vec<u8>),
    Revert(Vec<u8>),
}

impl Task<'_> {
    fn jump_to(&mut self, new_pc: UInt256) -> Result<(), VMError> {
        let from = self.input.index;
        self.input.index = new_pc.try_into().map_err(|_| VMError::TypeConversion)?;
        println!("Jumped from {:02X} to {:02X}", from, self.input.index);
        Ok(())
    }

    fn execute_single_instruction(
        &mut self,
        instruction: &Instruction,
        arg_option: Option<UInt256>,
    ) -> Result<InstructionResult, VMError> {
        let stack = &mut self.stack;
        print_instruction(instruction, arg_option);
        match *instruction {
            OP_ADD => {
                let result = stack.pop()? + stack.pop()?;
                stack.push(result);
            }
            OP_SUB => {
                let result = stack.pop()? - stack.pop()?;
                stack.push(result);
            }
            OP_LT => {
                let result = stack.pop()? < stack.pop()?;
                stack.push(UInt256::from_bool(result));
            }
            OP_EQ => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                println!("EQ -> {} {}", a, b);
                stack.push(UInt256::from_bool(a == b));
            }
            OP_SHR => {
                let shift = stack.pop()?;
                let value = stack.pop()?;
                let result = value >> shift;
                println!("SHR {} >> {} = {}", value, shift, result);
                stack.push(result);
            }
            OP_MLOAD => {
                let offset = stack.pop()?;
                let value = self.memory.load(offset)?;
                println!("MLOAD ({}) -> {}", offset, value);
                stack.push(value);
            }
            OP_MSTORE => {
                let offset = stack.pop()?;
                let value = stack.pop()?;
                self.memory.store(offset, value)?;
            }
            OP_CALLVALUE => {
                println!("CALLVALUE -> {}", self.message.value);
                stack.push(self.message.value);
            }
            OP_CALLDATASIZE => {
                stack.push(
                    self.message
                        .data
                        .len()
                        .try_into()
                        .map_err(|_| VMError::OutOfBounds)?,
                );
            }
            OP_CALLDATALOAD => {
                let offset = stack.pop()?;
                let index: usize = offset.try_into().map_err(|_| VMError::BadAccess)?;
                let end: usize = index + 32;
                let slice = self
                    .message
                    .data
                    .get(index..end)
                    .ok_or(VMError::BadAccess)?;
                let word = UInt256::from_be_slice(slice);
                println!("CALLDATALOAD (offset {}) -> {}", offset, word);
                self.stack.push(word);
            }
            OP_DUP1 => {
                stack.push(stack.peek(0)?);
            }
            OP_DUP2 => {
                stack.push(stack.peek(1)?);
            }
            OP_SWAP1 => {
                stack.swap(0, 1)?;
                println!("SWAP1 (old: {} new: {})", stack.peek(1)?, stack.peek(0)?);
            }
            OP_SWAP2 => {
                stack.swap(0, 2)?;
            }
            OP_ISZERO => {
                println!("ISZERO -> {}", stack.peek(0)? == UInt256::ZERO);
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
                println!("CODECOPY from {:?} to {:?}", from, to);
                // TODO: Does this index from input[0] or $PC or something else?
                self.memory.ensure_size(to.end);
                self.memory.bytes[to].copy_from_slice(&self.input.ops[from]);
            }
            OP_SLOAD => {
                let key = stack.pop()?;
                let value = self.storage.load(key);
                stack.push(value);
            }
            OP_JUMP => {
                let destination = stack.pop()?;
                self.jump_to(destination)?;
            }
            OP_JUMPI => {
                let destination = stack.pop()?;
                let condition = stack.pop()?;
                let is_truthy = condition != UInt256::ZERO;
                println!(
                    "JUMPI (condition: {} is_truthy: {})",
                    condition,
                    condition != UInt256::ZERO
                );
                if is_truthy {
                    self.jump_to(destination)?;
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
                println!("RETURN {}..{}", offset, offset + length);
                return Ok(InstructionResult::Return(self.memory.copy_out(range)?));
            }
            OP_REVERT => {
                // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-140.md
                // TODO: Should revert all actions?
                let offset = stack.pop()?;
                let length = stack.pop()?;
                let range = offset..offset + length;
                return Ok(InstructionResult::Revert(self.memory.copy_out(range)?));
            }
            // All push instructions:
            Instruction {
                op: 0x60..=0x7F, ..
            } => {
                let arg = arg_option.ok_or(VMError::BadArg)?;
                stack.push(arg);
            }
            _ => {
                return Err(VMError::BadOp(instruction.op));
            }
        }
        return Ok(InstructionResult::Continue);
    }
    fn execute(&mut self) -> Result<TaskResult, VMError> {
        // Does INSTRUCTIONS, take_op, inst and arg_option just
        // belong in some sort of Disasembler class?
        // Then the Disassembler wouldn't be responsible for erroring
        // on bad opcodes, but rather just returning an Unknown Op?
        let ops: HashMap<_, _> = INSTRUCTIONS
            .iter()
            .map(|instruction| (instruction.op, instruction))
            .collect();
        while let Some(op) = self.input.take_op() {
            let inst = ops.get(&op).ok_or(VMError::BadOp(op))?;
            let arg_option = self.input.take_arg(inst.arg)?;
            match self.execute_single_instruction(inst, arg_option)? {
                InstructionResult::Revert(data) => {
                    return Ok(TaskResult::Revert(data));
                }
                InstructionResult::Return(data) => {
                    return Ok(TaskResult::Return(data));
                }
                InstructionResult::Continue => {}
            }
        }
        Err(VMError::EndOfInstructions)
    }
}

#[derive(Default)]
pub struct InputManager {
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
    pub fn from_string(contents: &String) -> InputManager {
        InputManager::from_bytes(hex_string_as_vec_u8(contents))
    }

    pub fn from_bytes(ops: Vec<u8>) -> InputManager {
        InputManager { ops, index: 0 }
    }

    fn take_u8(&mut self) -> Result<u8, VMError> {
        if self.index < self.ops.len() {
            let value = self.ops[self.index];
            self.index += 1;
            Ok(value)
        } else {
            Err(VMError::BadArg)
        }
    }

    fn take(&mut self, size: usize) -> Result<UInt256, VMError> {
        let end = self.index + size;
        if end <= self.ops.len() {
            let bytes = &self.ops[self.index..end];
            self.index += size;
            Ok(UInt256::from_be_slice(bytes))
        } else {
            Err(VMError::BadArg)
        }
    }

    fn take_op(&mut self) -> Option<u8> {
        self.take_u8().ok()
    }

    fn take_arg(&mut self, arg_type: ArgType) -> Result<Option<UInt256>, VMError> {
        arg_type.size().map(|size| self.take(size)).transpose()
    }
}

// This should just end up a Display trait on a struct which has
// both Instruction and the optional arg on it.
pub fn print_instruction(inst: &Instruction, arg_option: Option<UInt256>) {
    match arg_option {
        Some(arg) => println!("{:02X}: {} ({})", inst.op, inst.name, arg),
        None => println!("{:02X}: {}", inst.op, inst.name),
    }
}

// This probably should be split out with InputManager to be a
// separate struct and maybe file which handles
// disassembly ahead of the VM.  The VM is passed a Disassembler?
pub fn dissemble(input: &mut InputManager) -> Result<(), VMError> {
    let ops: HashMap<_, _> = INSTRUCTIONS
        .iter()
        .map(|instruction| (instruction.op, instruction))
        .collect();

    while let Some(op) = input.take_op() {
        let inst = ops.get(&op).ok_or(VMError::BadOp(op))?;
        let arg_option = input.take_arg(inst.arg)?;
        print_instruction(inst, arg_option);
    }

    Ok(())
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ContractError {
    Revert(Vec<u8>),
    InternalError(VMError),
}

pub fn send_message_to_contract(
    message: Message,
    wrapper: InputManager,
) -> Result<(), ContractError> {
    let mut task = Task::new(wrapper, &message);
    let contract_bytes = match task
        .execute()
        .map_err(|e| ContractError::InternalError(e))?
    {
        TaskResult::Revert(data) => return Err(ContractError::Revert(data)),
        TaskResult::Return(bytes) => bytes,
    };
    println!("Got contract, executing!");
    let contract = InputManager::from_bytes(contract_bytes);
    let mut task = Task::new(contract, &message);
    match task
        .execute()
        .map_err(|e| ContractError::InternalError(e))?
    {
        TaskResult::Revert(data) => return Err(ContractError::Revert(data)),
        TaskResult::Return(data) => {
            println!("return Data: {:02X?}", data);
            return Ok(());
        }
    }
}
