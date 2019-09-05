use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::{Write, Cursor, Read};
use std::borrow::Borrow;
use crate::proto::{Encoder, Decoder};
use std::ops::Deref;

pub type ProtocolVersion = u32;

pub struct StartupMessage {
    protocol_version: ProtocolVersion,
    parameters: Vec<Parameter>,
}

impl Encoder for StartupMessage {
    fn encode(&self) -> Vec<u8> {
        let mut result = vec![0; 0];
        let _ = result.write_u32::<BigEndian>(self.protocol_version);
        let _ = result.write_u32::<BigEndian>(self.parameters.len() as u32);
        for param in self.parameters.as_slice() {
            let param_encoded = param.encode();
            let _ = result.write_u32::<BigEndian>(param_encoded.len() as u32);
            let _ = result.write(param_encoded.borrow());
        }
        return result;
    }
}

impl Decoder for StartupMessage {
    fn decode(&mut self, src: &[u8]) {
        let mut rdr = Cursor::new(src);
        self.protocol_version = rdr.read_u32::<BigEndian>().unwrap();
        let parameter_count = rdr.read_u32::<BigEndian>().unwrap();
        for i in 0..parameter_count {
            let param_size = rdr.read_u32::<BigEndian>().unwrap();
            let mut param_buf = vec![0; param_size as usize];
            rdr.read(&mut param_buf);
            self.parameters.push(Parameter::decode(param_buf.as_ref()));
        }
    }
}

pub struct Parameter {
    key: String,
    val: String,
}

impl Parameter {
    pub fn decode(src: &[u8]) -> Parameter {
        let mut p = Parameter{ key: "".to_string(), val: "".to_string() };
        p.decode(src);
        return p
    }
}

impl Encoder for Parameter {
    fn encode(&self) -> Vec<u8> {
        let mut result = vec![0; 0];
        let _ = result.write_u32::<BigEndian>(self.key.len() as u32).unwrap();
        let _ = result.write(self.key.as_ref());
        let _ = result.write_u32::<BigEndian>(self.val.len() as u32).unwrap();
        let _ = result.write(self.val.as_ref());
        return result;
    }
}

impl Decoder for Parameter {
    fn decode(&mut self, src: &[u8]) {
        let mut rdr = Cursor::new(src);
        let key_size = rdr.read_u32::<BigEndian>().unwrap();
        let mut key = vec![0; key_size as usize];
        let res = rdr.read_exact(key.as_mut());
        self.key = String::from_utf8(key).unwrap();

        let val_size = rdr.read_u32::<BigEndian>().unwrap();
        let mut val = vec![0; val_size as usize];
        let res = rdr.read_exact(val.as_mut());
        self.val = String::from_utf8(val).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup_message_encode_decode() {
        let input = StartupMessage {
            protocol_version: 123456,
            parameters: vec![
                Parameter {
                    key: "user".to_string(),
                    val: "postgres".to_string(),
                },
                Parameter {
                    key: "database".to_string(),
                    val: "my_db".to_string(),
                }
            ],
        };
        let encoded = input.encode();
        assert!(encoded.len() > 0);
        let mut decoded = StartupMessage{ protocol_version: 0, parameters: vec![] };
        decoded.decode(encoded.as_ref());
        assert_eq!(decoded.protocol_version, input.protocol_version);
    }

    #[test]
    fn test_parameter_encode_decode() {
        let input = Parameter { key: "user".to_string(), val: "postgres".to_string() };
        let encoded = input.encode();
        assert!(encoded.len() > 0);
        let mut decoded = Parameter{ key: "".to_string(), val: "".to_string() };
        decoded.decode(encoded.as_ref());
        assert_eq!(decoded.key, input.key);
        assert_eq!(decoded.val, input.val);
    }
}