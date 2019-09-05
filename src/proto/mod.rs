pub mod startup_message;

trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

trait Decoder {
    fn decode(&mut self, src: &[u8]);
}