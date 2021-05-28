#![cfg_attr(rustfmt, rustfmt_skip)]

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArgType {
    Void,
    U8,
    U16,
    U24,
    U32,
    U40,
    U48,
    U56,
    U64,
    U128,
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
pub const OP_SHL: Instruction = Instruction { op: 0x1b, name: "SHL", arg: ArgType::Void };
pub const OP_SHR: Instruction = Instruction { op: 0x1c, name: "SHR", arg: ArgType::Void };
pub const OP_SAR: Instruction = Instruction { op: 0x1d, name: "SAR", arg: ArgType::Void };
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
pub const OP_PUSH2: Instruction = Instruction { op: 0x61, name: "PUSH2", arg: ArgType::U16 };
pub const OP_PUSH3: Instruction = Instruction { op: 0x62, name: "PUSH3", arg: ArgType::U24 };
pub const OP_PUSH4: Instruction = Instruction { op: 0x63, name: "PUSH4", arg: ArgType::U32 };
pub const OP_PUSH5: Instruction = Instruction { op: 0x64, name: "PUSH5", arg: ArgType::U40 };
pub const OP_PUSH6: Instruction = Instruction { op: 0x65, name: "PUSH6", arg: ArgType::U48 };
pub const OP_PUSH7: Instruction = Instruction { op: 0x66, name: "PUSH7", arg: ArgType::U56 };
pub const OP_PUSH8: Instruction = Instruction { op: 0x67, name: "PUSH8", arg: ArgType::U64 };
pub const OP_PUSH16: Instruction = Instruction { op: 0x6F, name: "PUSH16", arg: ArgType::U128 };
pub const OP_DUP1: Instruction = Instruction { op: 0x80, name: "DUP1", arg: ArgType::Void };
pub const OP_DUP2: Instruction = Instruction { op: 0x81, name: "DUP2", arg: ArgType::Void };
pub const OP_DUP3: Instruction = Instruction { op: 0x82, name: "DUP3", arg: ArgType::Void };
pub const OP_DUP4: Instruction = Instruction { op: 0x83, name: "DUP4", arg: ArgType::Void };
pub const OP_DUP5: Instruction = Instruction { op: 0x84, name: "DUP5", arg: ArgType::Void };
pub const OP_DUP6: Instruction = Instruction { op: 0x85, name: "DUP6", arg: ArgType::Void };
pub const OP_DUP7: Instruction = Instruction { op: 0x86, name: "DUP7", arg: ArgType::Void };
pub const OP_DUP8: Instruction = Instruction { op: 0x87, name: "DUP8", arg: ArgType::Void };
pub const OP_DUP9: Instruction = Instruction { op: 0x88, name: "DUP9", arg: ArgType::Void };
pub const OP_DUP10: Instruction = Instruction { op: 0x89, name: "DUP10", arg: ArgType::Void };
pub const OP_DUP11: Instruction = Instruction { op: 0x8A, name: "DUP11", arg: ArgType::Void };
pub const OP_DUP12: Instruction = Instruction { op: 0x8B, name: "DUP12", arg: ArgType::Void };
pub const OP_DUP13: Instruction = Instruction { op: 0x8C, name: "DUP13", arg: ArgType::Void };
pub const OP_DUP14: Instruction = Instruction { op: 0x8D, name: "DUP14", arg: ArgType::Void };
pub const OP_DUP15: Instruction = Instruction { op: 0x8E, name: "DUP15", arg: ArgType::Void };
pub const OP_DUP16: Instruction = Instruction { op: 0x8F, name: "DUP16", arg: ArgType::Void };
pub const OP_SWAP1: Instruction = Instruction { op: 0x90, name: "SWAP1", arg: ArgType::Void };
pub const OP_SWAP2: Instruction = Instruction { op: 0x91, name: "SWAP2", arg: ArgType::Void };
pub const OP_SWAP3: Instruction = Instruction { op: 0x92, name: "SWAP3", arg: ArgType::Void };
pub const OP_SWAP4: Instruction = Instruction { op: 0x93, name: "SWAP4", arg: ArgType::Void };
pub const OP_SWAP5: Instruction = Instruction { op: 0x94, name: "SWAP5", arg: ArgType::Void };
pub const OP_SWAP6: Instruction = Instruction { op: 0x95, name: "SWAP6", arg: ArgType::Void };
pub const OP_SWAP7: Instruction = Instruction { op: 0x96, name: "SWAP7", arg: ArgType::Void };
pub const OP_SWAP8: Instruction = Instruction { op: 0x97, name: "SWAP8", arg: ArgType::Void };
pub const OP_SWAP9: Instruction = Instruction { op: 0x98, name: "SWAP9", arg: ArgType::Void };
pub const OP_SWAP10: Instruction = Instruction { op: 0x99, name: "SWAP10", arg: ArgType::Void };
pub const OP_SWAP11: Instruction = Instruction { op: 0x9A, name: "SWAP11", arg: ArgType::Void };
pub const OP_SWAP12: Instruction = Instruction { op: 0x9B, name: "SWAP12", arg: ArgType::Void };
pub const OP_SWAP13: Instruction = Instruction { op: 0x9C, name: "SWAP13", arg: ArgType::Void };
pub const OP_SWAP14: Instruction = Instruction { op: 0x9D, name: "SWAP14", arg: ArgType::Void };
pub const OP_SWAP15: Instruction = Instruction { op: 0x9E, name: "SWAP15", arg: ArgType::Void };
pub const OP_SWAP16: Instruction = Instruction { op: 0x9F, name: "SWAP16", arg: ArgType::Void };
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
pub const OP_REVERT: Instruction = Instruction { op: 0xfd, name: "REVERT", arg: ArgType::Void };
pub const OP_INVALID: Instruction = Instruction { op: 0xfe, name: "INVALID", arg: ArgType::Void };
pub const OP_SELFDESTRUCT: Instruction = Instruction { op: 0xff, name: "SELFDESTRUCT", arg: ArgType::Void };

pub const INSTRUCTIONS: [Instruction; 112] = [
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
    OP_SHL,
    OP_SHR,
    OP_SAR,
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
    OP_PUSH2,
    OP_PUSH3,
    OP_PUSH4,
    OP_PUSH5,
    OP_PUSH6,
    OP_PUSH7,
    OP_PUSH8,
    OP_PUSH16,
    OP_DUP1,
    OP_DUP2,
    OP_DUP3,
    OP_DUP4,
    OP_DUP5,
    OP_DUP6,
    OP_DUP7,
    OP_DUP8,
    OP_DUP9,
    OP_DUP10,
    OP_DUP11,
    OP_DUP12,
    OP_DUP13,
    OP_DUP14,
    OP_DUP15,
    OP_DUP16,
    OP_SWAP1,
    OP_SWAP2,
    OP_SWAP3,
    OP_SWAP4,
    OP_SWAP5,
    OP_SWAP6,
    OP_SWAP7,
    OP_SWAP8,
    OP_SWAP9,
    OP_SWAP10,
    OP_SWAP11,
    OP_SWAP12,
    OP_SWAP13,
    OP_SWAP14,
    OP_SWAP15,
    OP_SWAP16,
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
    OP_REVERT,
    OP_INVALID,
    OP_SELFDESTRUCT,
];
