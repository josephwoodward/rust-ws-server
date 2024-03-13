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
    TEXT = 1,
    CLOSE = 8,
    PING = 9,
    PONG = 10,
}

impl OpCode {
    pub fn from_u8(v: u8) -> OpCode {
        match v & 0x0F {
            1 => OpCode::TEXT,
            8 => OpCode::CLOSE,
            9 => OpCode::PING,
            10 => OpCode::PONG,
            _ => panic!("Unknown op code: {}", v),
        }
    }
    pub fn to_u8(oc: &OpCode) -> u8 {
        *oc as u8
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::TEXT => write!(f, "TEXT"),
            OpCode::CLOSE => write!(f, "CLOSE"),
            OpCode::PING => write!(f, "PING"),
            OpCode::PONG => write!(f, "PONG"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::request::OpCode;

    #[test]
    fn test_op_code_parsing() {
        assert!(matches!(OpCode::from_u8(129), OpCode::TEXT));
        assert!(matches!(OpCode::from_u8(136), OpCode::CLOSE));
        assert!(matches!(OpCode::from_u8(137), OpCode::PING));
        assert!(matches!(OpCode::from_u8(138), OpCode::PONG));
    }
}
