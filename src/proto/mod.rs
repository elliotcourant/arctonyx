pub mod startup_message;

trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

macro_rules! impl_encoder {
    ($T:ident) => {
        impl Encoder for $T {
            fn encode(&self) -> Vec<u8> { return self.encode(); }
        }
    }
}

trait Decoder {
    fn decode(&mut self, src: Vec<u8>);
}

macro_rules! impl_decoder {
    ($T:ident) => {
        impl Decoder for $T {
            fn decode(&mut self, src: Vec<u8>) { self.decode(src); }
        }
    }
}

use crate::proto::startup_message::StartupMessage;
impl_encoder!(StartupMessage);
impl_decoder!(StartupMessage);