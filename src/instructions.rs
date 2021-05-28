#![cfg_attr(rustfmt, rustfmt_skip)]

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArgType {
    Void,
    U8,
    U16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArgValue {
    Void,
    U8(u8),
    U16(u16),
}

impl fmt::Display for ArgValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgValue::Void => Ok(()),
            ArgValue::U8(value) => {
                write!(f, "({})", value)
            }
            ArgValue::U16(value) => {
                write!(f, "({})", value)
            }
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub op: u8,
    pub name: &'static str,
    pub arg: ArgType,
}

impl Instruction {
    pub fn new(op: u8, name: &'static str) -> Instruction {
        Instruction {
            op,
            name,
            arg: ArgType::Void,
        }
    }

    pub fn new_with_arg(op: u8, name: &'static str, arg: ArgType) -> Instruction {
        Instruction { op, name, arg }
    }
}

pub const OP_STOP: Instruction = Instruction { op: 0x00, name: "STOP", arg: ArgType::Void };
pub const OP_ADD: Instruction = Instruction { op: 0x01, name: "ADD", arg: ArgType::Void };
pub const OP_MUL: Instruction = Instruction { op: 0x02, name: "MUL", arg: ArgType::Void };
pub const OP_SUB: Instruction = Instruction { op: 0x03, name: "SUB", arg: ArgType::Void };
pub const OP_DIV: Instruction = Instruction { op: 0x04, name: "DIV", arg: ArgType::Void };
pub const OP_SDIV: Instruction = Instruction { op: 0x05, name: "SDIV", arg: ArgType::Void };
pub const OP_MOD: Instruction = Instruction { op: 0x06, name: "MOD", arg: ArgType::Void };
pub const OP_SMOD: Instruction = Instruction { op: 0x07, name: "SMOD", arg: ArgType::Void };
pub const OP_ADDMOD: Instruction = Instruction { op: 0x08, name: "ADDMOD", arg: ArgType::Void };
pub const OP_MULMOD: Instruction = Instruction { op: 0x09, name: "MULMOD", arg: ArgType::Void };
pub const OP_EXP: Instruction = Instruction { op: 0x0a, name: "EXP", arg: ArgType::Void };
pub const OP_SIGNEXTEND: Instruction = Instruction { op: 0x0b, name: "SIGNEXTEND", arg: ArgType::Void };
pub const OP_LT: Instruction = Instruction { op: 0x10, name: "LT", arg: ArgType::Void };
pub const OP_GT: Instruction = Instruction { op: 0x11, name: "GT", arg: ArgType::Void };
pub const OP_SLT: Instruction = Instruction { op: 0x12, name: "SLT", arg: ArgType::Void };
pub const OP_SGT: Instruction = Instruction { op: 0x13, name: "SGT", arg: ArgType::Void };
pub const OP_EQ: Instruction = Instruction { op: 0x14, name: "EQ", arg: ArgType::Void };
pub const OP_ISZERO: Instruction = Instruction { op: 0x15, name: "ISZERO", arg: ArgType::Void };
pub const OP_AND: Instruction = Instruction { op: 0x16, name: "AND", arg: ArgType::Void };
pub const OP_OR: Instruction = Instruction { op: 0x17, name: "OR", arg: ArgType::Void };
pub const OP_XOR: Instruction = Instruction { op: 0x18, name: "XOR", arg: ArgType::Void };
pub const OP_NOT: Instruction = Instruction { op: 0x19, name: "NOT", arg: ArgType::Void };
pub const OP_BYTE: Instruction = Instruction { op: 0x1a, name: "BYTE", arg: ArgType::Void };
pub const OP_SHA3: Instruction = Instruction { op: 0x20, name: "SHA3", arg: ArgType::Void };
pub const OP_ADDRESS: Instruction = Instruction { op: 0x30, name: "ADDRESS", arg: ArgType::Void };
pub const OP_BALANCE: Instruction = Instruction { op: 0x31, name: "BALANCE", arg: ArgType::Void };
pub const OP_ORIGIN: Instruction = Instruction { op: 0x32, name: "ORIGIN", arg: ArgType::Void };
pub const OP_CALLER: Instruction = Instruction { op: 0x33, name: "CALLER", arg: ArgType::Void };
pub const OP_CALLVALUE: Instruction = Instruction { op: 0x34, name: "CALLVALUE", arg: ArgType::Void };
pub const OP_CALLDATALOAD: Instruction = Instruction { op: 0x35, name: "CALLDATALOAD", arg: ArgType::Void };
pub const OP_CALLDATASIZE: Instruction = Instruction { op: 0x36, name: "CALLDATASIZE", arg: ArgType::Void };
pub const OP_CALLDATACOPY: Instruction = Instruction { op: 0x37, name: "CALLDATACOPY", arg: ArgType::Void };
pub const OP_CODESIZE: Instruction = Instruction { op: 0x38, name: "CODESIZE", arg: ArgType::Void };
pub const OP_CODECOPY: Instruction = Instruction { op: 0x39, name: "CODECOPY", arg: ArgType::Void };
pub const OP_GASPRICE: Instruction = Instruction { op: 0x3a, name: "GASPRICE", arg: ArgType::Void };
pub const OP_EXTCODESIZE: Instruction = Instruction { op: 0x3b, name: "EXTCODESIZE", arg: ArgType::Void };
pub const OP_EXTCODECOPY: Instruction = Instruction { op: 0x3c, name: "EXTCODECOPY", arg: ArgType::Void };
pub const OP_BLOCKHASH: Instruction = Instruction { op: 0x40, name: "BLOCKHASH", arg: ArgType::Void };
pub const OP_COINBASE: Instruction = Instruction { op: 0x41, name: "COINBASE", arg: ArgType::Void };
pub const OP_TIMESTAMP: Instruction = Instruction { op: 0x42, name: "TIMESTAMP", arg: ArgType::Void };
pub const OP_NUMBER: Instruction = Instruction { op: 0x43, name: "NUMBER", arg: ArgType::Void };
pub const OP_DIFFICULTY: Instruction = Instruction { op: 0x44, name: "DIFFICULTY", arg: ArgType::Void };
pub const OP_GASLIMIT: Instruction = Instruction { op: 0x45, name: "GASLIMIT", arg: ArgType::Void };
pub const OP_POP: Instruction = Instruction { op: 0x50, name: "POP", arg: ArgType::Void };
pub const OP_MLOAD: Instruction = Instruction { op: 0x51, name: "MLOAD", arg: ArgType::Void };
pub const OP_MSTORE: Instruction = Instruction { op: 0x52, name: "MSTORE", arg: ArgType::Void };
pub const OP_MSTORE8: Instruction = Instruction { op: 0x53, name: "MSTORE8", arg: ArgType::Void };
pub const OP_SLOAD: Instruction = Instruction { op: 0x54, name: "SLOAD", arg: ArgType::Void };
pub const OP_SSTORE: Instruction = Instruction { op: 0x55, name: "SSTORE", arg: ArgType::Void };
pub const OP_JUMP: Instruction = Instruction { op: 0x56, name: "JUMP", arg: ArgType::Void };
pub const OP_JUMPI: Instruction = Instruction { op: 0x57, name: "JUMPI", arg: ArgType::Void };
pub const OP_PC: Instruction = Instruction { op: 0x58, name: "PC", arg: ArgType::Void };
pub const OP_MSIZE: Instruction = Instruction { op: 0x59, name: "MSIZE", arg: ArgType::Void };
pub const OP_GAS: Instruction = Instruction { op: 0x5a, name: "GAS", arg: ArgType::Void };
pub const OP_JUMPDEST: Instruction = Instruction { op: 0x5b, name: "JUMPDEST", arg: ArgType::Void };
pub const OP_PUSH1: Instruction = Instruction { op: 0x60, name: "PUSH1", arg: ArgType::U8 };
// 0x60 -- 0x7f	PUSH*
pub const OP_DUP: Instruction = Instruction { op: 0x80, name: "DUP", arg: ArgType::Void };
// 0x80 -- 0x8f	DUP*
// 0x90 -- 0x9f	SWAP*
pub const OP_LOG0: Instruction = Instruction { op: 0xa0, name: "LOG0", arg: ArgType::Void };
pub const OP_LOG1: Instruction = Instruction { op: 0xa1, name: "LOG1", arg: ArgType::Void };
pub const OP_LOG2: Instruction = Instruction { op: 0xa2, name: "LOG2", arg: ArgType::Void };
pub const OP_LOG3: Instruction = Instruction { op: 0xa3, name: "LOG3", arg: ArgType::Void };
pub const OP_LOG4: Instruction = Instruction { op: 0xa4, name: "LOG4", arg: ArgType::Void };
pub const OP_CREATE: Instruction = Instruction { op: 0xf0, name: "CREATE", arg: ArgType::Void };
pub const OP_CALL: Instruction = Instruction { op: 0xf1, name: "CALL", arg: ArgType::Void };
pub const OP_CALLCODE: Instruction = Instruction { op: 0xf2, name: "CALLCODE", arg: ArgType::Void };
pub const OP_RETURN: Instruction = Instruction { op: 0xf3, name: "RETURN", arg: ArgType::Void };
pub const OP_DELEGATECALL: Instruction = Instruction { op: 0xf4, name: "DELEGATECALL", arg: ArgType::Void };
pub const OP_INVALID: Instruction = Instruction { op: 0xfe, name: "INVALID", arg: ArgType::Void };
pub const OP_SELFDESTRUCT: Instruction = Instruction { op: 0xff, name: "SELFDESTRUCT", arg: ArgType::Void };

pub const INSTRUCTIONS: [Instruction; 69] = [
    OP_STOP,
    OP_ADD,
    OP_MUL,
    OP_SUB,
    OP_DIV,
    OP_SDIV,
    OP_MOD,
    OP_SMOD,
    OP_ADDMOD,
    OP_MULMOD,
    OP_EXP,
    OP_SIGNEXTEND,
    OP_LT,
    OP_GT,
    OP_SLT,
    OP_SGT,
    OP_EQ,
    OP_ISZERO,
    OP_AND,
    OP_OR,
    OP_XOR,
    OP_NOT,
    OP_BYTE,
    OP_SHA3,
    OP_ADDRESS,
    OP_BALANCE,
    OP_ORIGIN,
    OP_CALLER,
    OP_CALLVALUE,
    OP_CALLDATALOAD,
    OP_CALLDATASIZE,
    OP_CALLDATACOPY,
    OP_CODESIZE,
    OP_CODECOPY,
    OP_GASPRICE,
    OP_EXTCODESIZE,
    OP_EXTCODECOPY,
    OP_BLOCKHASH,
    OP_COINBASE,
    OP_TIMESTAMP,
    OP_NUMBER,
    OP_DIFFICULTY,
    OP_GASLIMIT,
    OP_POP,
    OP_MLOAD,
    OP_MSTORE,
    OP_MSTORE8,
    OP_SLOAD,
    OP_SSTORE,
    OP_JUMP,
    OP_JUMPI,
    OP_PC,
    OP_MSIZE,
    OP_GAS,
    OP_JUMPDEST,
    OP_PUSH1,
    // OP_PUSH*,
    OP_DUP,
    // OP_DUP*,
    // OP_SWAP*,
    OP_LOG0,
    OP_LOG1,
    OP_LOG2,
    OP_LOG3,
    OP_LOG4,
    OP_CREATE,
    OP_CALL,
    OP_CALLCODE,
    OP_RETURN,
    OP_DELEGATECALL,
    OP_INVALID,
    OP_SELFDESTRUCT,
];
