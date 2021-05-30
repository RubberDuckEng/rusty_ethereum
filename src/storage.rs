use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::fs;

use crate::UInt256;

#[derive(Default)]
pub struct Storage {
    // address: UInt256,
}

#[derive(Serialize, Deserialize, Default)]
pub struct StorageFile {
    key_pairs: HashMap<String, String>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StorageError {
    CantSerialize,
}

impl fmt::Debug for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::CantSerialize => write!(f, "CantSerialize"),
        }
    }
}

impl Storage {
    fn filename(&self) -> String {
        String::from("storage.txt")
    }

    pub fn load_storage_file(&self) -> StorageFile {
        match fs::read_to_string(self.filename()) {
            Ok(contents) => serde_json::from_str(&contents).expect("parse failure"),
            Err(e) => match e.kind() {
                // Return Err for non-recoverable errors?
                _ => StorageFile::default(),
            },
        }
    }

    pub fn load(&self, key: UInt256) -> Result<UInt256, StorageError> {
        let storage_file = self.load_storage_file();
        let key_string = format!("{}", key);
        match storage_file.key_pairs.get(&key_string) {
            Some(value_str) => Ok(UInt256::from_string(&value_str)),
            None => Ok(UInt256::ZERO),
        }
    }

    pub fn store(&mut self, key: UInt256, value: UInt256) -> Result<(), StorageError> {
        let mut storage_file = self.load_storage_file();
        let key_string = format!("{}", key);
        let value_string = format!("{}", value);
        storage_file.key_pairs.insert(key_string, value_string);

        let write_contents: String =
            serde_json::to_string(&storage_file).map_err(|_| StorageError::CantSerialize)?;
        fs::write(self.filename(), write_contents).expect("write fail");
        Ok(())
    }
}
