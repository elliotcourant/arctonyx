use crate::proto::{Encoder, Decoder};

pub enum Type {
    Ok = 0,
    CleartextPassword = 3,
    MD5Password = 5,
}

pub struct Authentication {
    pub typ: Type,
    pub salt: [u8; 4],
}

impl Encoder for Authentication {
    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }
}

impl Decoder for Authentication {
    fn decode(&mut self, src: &[u8]) {
        unimplemented!()
    }
}