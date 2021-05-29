use crate::UInt256;

use sha3::Digest;

#[derive(Default)]
pub struct Message {
    pub value: UInt256, // message funds in wei
    pub caller: UInt256,
    // First four bytes should be signature of method being called, e.g.
    // data[..4] = bytes4(keccak256(“add(uint256,uint256)”));
    pub data: Vec<u8>,
}

impl Message {
    pub fn new_call(method_name: &str) -> Message {
        let mut data = vec![0u8; 32];
        // TODO: I suspect this encoding is incorrect.
        // This has not been tested!
        let encoded_name = sha3::Sha3_256::digest(method_name.as_bytes());
        data[..4].copy_from_slice(&encoded_name[..4]);

        Message {
            value: UInt256::ZERO, // Zero wei?
            caller: UInt256::ZERO,
            data: data,
        }
    }
}
