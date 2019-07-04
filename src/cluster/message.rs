use std::mem;
use serde::{Serialize, Deserialize};

pub const MESSAGE_SIZE: usize = mem::size_of::<Message>();

#[derive(Serialize, Deserialize)]
pub enum Message {
    AskId,
    TellId(usize),
}

impl Message {
    pub fn serialized(&self) -> [u8; MESSAGE_SIZE] {
        let mut buffer = [0; MESSAGE_SIZE];
        let serialized = &bincode::serialize(self).unwrap();
        buffer[..serialized.len()].copy_from_slice(serialized);
        buffer
    }
}
