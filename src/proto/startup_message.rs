pub type ProtocolVersion = u32;

pub struct StartupMessage {
    protocol_version: ProtocolVersion,
    parameters: Vec<Parameter>,
}

pub struct Parameter {
    key: String,
    val: String,
}

impl Parameter {
    pub fn encode(&self) -> Vec<u8> {
        let mut result = vec![0; 8 + self.key.len() + self.val.len()];
        return result.to_vec();
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
    fn test_parameter_encode() {
        let param = Parameter { key: String::from("1"), val: String::from("1") };
        let result = param.encode();
        println!("test")
    }
}