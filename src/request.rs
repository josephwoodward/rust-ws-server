pub struct Request {
    pub path: String,
    pub method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        let s = String::from("boo");
        todo!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(_: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
