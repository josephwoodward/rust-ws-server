use std::string::ParseError;

pub struct Request {
    pub path: String,
    // pub method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(_: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
