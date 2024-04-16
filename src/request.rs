use std::{fmt, string::ParseError};

pub struct Request {
    pub path: String,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(_: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum OpCode {
    Text = 1,
    Close = 8,
    Ping = 9,
    Pong = 10,
}

impl OpCode {
    pub fn from_u8(v: u8) -> OpCode {
        match v & 0x0F {
            1 => OpCode::Text,
            8 => OpCode::Close,
            9 => OpCode::Ping,
            10 => OpCode::Pong,
            _ => panic!("Unknown op code: {}", v),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::Text => write!(f, "TEXT"),
            OpCode::Close => write!(f, "CLOSE"),
            OpCode::Ping => write!(f, "PING"),
            OpCode::Pong => write!(f, "PONG"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::request::OpCode;

    #[test]
    fn test_op_code_parsing() {
        assert!(matches!(OpCode::from_u8(129), OpCode::Text));
        assert!(matches!(OpCode::from_u8(136), OpCode::Close));
        assert!(matches!(OpCode::from_u8(137), OpCode::Ping));
        assert!(matches!(OpCode::from_u8(138), OpCode::Pong));
    }
}
