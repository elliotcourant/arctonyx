use crate::proto::authentication::{Authentication, Type};
use crate::proto::startup_message::StartupMessage;
use std::io::Cursor;
use byteorder::{ReadBytesExt, BigEndian};
use std::error::Error;

pub mod startup_message;
pub mod authentication;
mod headers;

trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

trait Decoder {
    fn decode(&mut self, src: &[u8]);
}

pub enum Message {
    Authentication,
    StartupMessage,
}

pub enum Backend {
    Authentication(Authentication)
}

pub enum Frontend {
    StartupMessage(StartupMessage)
}

pub struct Header {
    message: Message,
    size: u32,
}

pub type ProtocolError = String;

macro_rules! proto_err {
    ($MSG:expr) => {
        Err($MSG.to_string())
    };
}

impl Header {
    pub fn read_backend_header(src: &[u8; 5]) -> Result<Header, ProtocolError> {
        let typ = src[0];
        let size = Cursor::new(src[1..].to_vec()).read_u32::<BigEndian>().unwrap() - 4;
        let message: Message = match typ {
            headers::AUTHENTICATION => Message::Authentication,
            _ => return proto_err!("unrecognized header character")
        };
        Ok(Header {
            message,
            size,
        })
    }
}