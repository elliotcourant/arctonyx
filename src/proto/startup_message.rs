use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::{Write, Cursor, Read};
use std::borrow::Borrow;

pub type ProtocolVersion = u32;

pub struct StartupMessage {
    protocol_version: ProtocolVersion,
    parameters: Vec<Parameter>,
}

impl StartupMessage {
    pub fn encode(&self) -> Vec<u8> {
        let mut result = vec![0; 0];
        result.write_u32::<BigEndian>(self.protocol_version);
        result.write_u32::<BigEndian>(self.parameters.len() as u32);
        for param in self.parameters.as_slice() {
            let param_encoded = param.encode();
            result.write_u32::<BigEndian>(param_encoded.len() as u32);
            result.write(param_encoded.borrow());
        }
        return result.to_vec();
    }
}

pub struct Parameter {
    key: String,
    val: String,
}

impl Parameter {
    pub fn encode(&self) -> Vec<u8> {
        let mut result = vec![0; 0];
        result.write_u32::<BigEndian>(self.key.len() as u32).unwrap();
        result.write(self.key.as_ref());
        result.write_u32::<BigEndian>(self.val.len() as u32).unwrap();
        result.write(self.val.as_ref());
        return result.to_vec();
    }

    fn decode(&mut self, src: Vec<u8>) {
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
    fn test_startup() {
        let startup: StartupMessage;
    }

    #[test]
    fn test_startup_message_encode_decode() {
        let startup = StartupMessage {
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
        let encoded = startup.encode();
        assert!(encoded.len() > 0);
    }

    #[test]
    fn test_parameter_encode_decode() {
        let param = Parameter { key: String::from("user"), val: String::from("postgres") };
        let result = param.encode();
        let mut decoded = Parameter { key: "".to_string(), val: "".to_string() };
        decoded.decode(result);
        assert_eq!(decoded.key, param.key);
        assert_eq!(decoded.val, param.val);
    }
}