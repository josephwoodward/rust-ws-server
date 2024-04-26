use std::fmt;

pub struct Frame {
    pub is_final: bool,
    pub op_code: OpCode,
    pub is_masked: bool,
    pub masking_key: Option<[u8; 4]>,
    pub payload_length: u8,
    pub payload: Option<Vec<u8>>,
}

impl Frame {
    /// Creates a new websocket frame
    pub fn new(head: [u8; 2]) -> Self {
        let mut f = Self {
            op_code: OpCode::from_u8(head[0]),
            is_final: (head[0] & 0x80) == 0x00,
            is_masked: (head[1] & 0x80) == 0x80,
            payload_length: head[1] & 0x7F,
            masking_key: None,
            payload: None,
        };

        if f.payload_length > 0 {
            f.payload = Some(vec![0; f.payload_length.into()]);
        }

        f
    }

    /// Creates a new websocket text based frame
    pub fn text(msg: String) -> Self {
        Self {
            op_code: OpCode::Text,
            is_final: true,
            is_masked: false,
            payload_length: msg.len().to_ne_bytes()[0],
            masking_key: None,
            payload: Some(msg.as_bytes().to_vec()),
        }
    }

    /// Creates a new websocket close based frame
    pub fn close() -> Self {
        Self {
            op_code: OpCode::Close,
            is_final: true,
            is_masked: false,
            payload_length: 0,
            masking_key: None,
            payload: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Text = 0x1,
    Close = 0x8,
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
    use crate::frame::{Frame, OpCode};

    #[test]
    fn test_op_code_parsing() {
        assert!(matches!(OpCode::from_u8(0x1), OpCode::Text));
        assert!(matches!(OpCode::from_u8(0x8), OpCode::Close));
        assert!(matches!(OpCode::from_u8(137), OpCode::Ping));
        assert!(matches!(OpCode::from_u8(138), OpCode::Pong));
    }

    #[test]
    fn create_text_frame() {
        let expected = "Hello Mike!";
        let f = Frame::text(expected.to_string());
        assert!(!f.is_masked);
        assert!(f.is_final);
        assert_eq!(f.op_code, OpCode::Text);
        assert_eq!(usize::from(f.payload_length), expected.len());
    }
}
