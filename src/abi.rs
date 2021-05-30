use crate::UInt256;

use tiny_keccak::{Hasher, Keccak};

#[derive(Default)]
pub struct Message {
    pub value: UInt256, // message funds in wei
    pub caller: UInt256,
    // First four bytes should be signature of method being called, e.g.
    // data[..4] = bytes4(keccak256(“add(uint256,uint256)”));
    pub data: Vec<u8>,
}

fn method_signature(method_name: &str) -> [u8; 4] {
    //format is name(input,input,input...)
    let mut full_hash = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(method_name.as_bytes());
    hasher.finalize(&mut full_hash);

    let mut data = [0u8; 4];
    println!("{}: {:02X?}", method_name, full_hash);
    data.copy_from_slice(&full_hash[..4]);
    return data;
}

impl Message {
    pub fn new_call(method_name: &str) -> Message {
        let mut data = vec![0u8; 32];
        data[..4].copy_from_slice(&method_signature(method_name));

        Message {
            value: UInt256::ZERO, // Zero wei?
            caller: UInt256::ZERO,
            data: data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::method_signature;

    #[test]
    fn name_encoding() {
        fn check_encoding(method_name: &str, hex_string: &str) -> bool {
            let sig_bytes = method_signature(method_name);
            let sig_int = u32::from_be_bytes(sig_bytes);
            let hex_int = u32::from_str_radix(hex_string, 16).ok().unwrap();
            sig_int == hex_int
        }

        assert!(check_encoding("count()", "06661abd"));
        assert!(check_encoding("dec()", "b3bcfa82"));
        assert!(check_encoding("get()", "6d4ce63c"));
        assert!(check_encoding("inc()", "371303c0"));
    }
}
