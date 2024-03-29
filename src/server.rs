use sha1::{Digest, Sha1};
use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
    usize,
};

use crate::request::OpCode;

pub struct Frame {
    is_final: bool,
    op_code: OpCode,
    is_masked: bool,
    payload: Option<Vec<u8>>,
    length: u8,
}

impl Frame {
    pub fn new(op_code: OpCode, is_final: bool, is_masked: bool) -> Self {
        if is_masked {
            println!("payload is masked");
        }
        if is_final {
            println!("payload is final");
        }

        Self {
            op_code,
            is_final,
            is_masked,
            payload: None,
            length: 0,
        }
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let listener = match TcpListener::bind(&self.addr) {
            Ok(l) => l,
            Err(e) => return Err(e.to_string()),
        };

        println!("server running and listening on: {}", &self.addr);

        for stream in listener.incoming() {
            let s = stream.expect("failed to read from TCP stream");
            self.upgrade_connection(s);
        }

        return Ok(());
    }

    fn upgrade_connection(&mut self, mut stream: TcpStream) {
        // initial HTTP websocket haneshake
        // Opening handshake: https://datatracker.ietf.org/doc/html/rfc6455#section-1.3
        // GET /chat HTTP/1.1
        // Host: server.example.com
        // Upgrade: websocket
        // Connection: Upgrade
        // Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
        // Origin: http://example.com
        // Sec-WebSocket-Protocol: chat, superchat
        // Sec-WebSocket-Version: 13

        // HTTP/1.1 101 Switching Protocols
        // Upgrade: websocket
        // Connection: Upgrade
        // Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=
        // From https://tools.ietf.org/html/rfc6455#section-4.2.2
        let initial_handshake: Vec<String> = BufReader::new(&mut stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if initial_handshake[0] != "GET /ws HTTP/1.1" {
            let mut w = BufWriter::new(&mut stream);
            w.write("HTTP/1.1 404 Not Found".as_bytes()).unwrap();
            w.flush().unwrap();
        } else if let Some(key) = find_websocket_key(initial_handshake) {
            let hash = generate_hash(key);
            let mut w = BufWriter::new(&mut stream);
            w.write(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {hash}\r\n\r\n").as_bytes())
                .unwrap();
            w.flush().unwrap();

            println!("switching protocols");
        }

        loop {
            let mut head = vec![0u8; 2];
            let _ = stream
                .read_exact(&mut head)
                .expect("could not read from buffer");

            let is_masked = (head[1] & 0x80) == 0x80;
            let is_final = (head[0] & 0x80) == 0x00;
            let mut f = Frame::new(OpCode::from_u8(head[0]), is_final, is_masked);

            match f.op_code {
                OpCode::TEXT => {
                    f.length = head[1] & 0x7F;
                    if f.length > 0 {
                        f.payload = Some(vec![0; f.length.into()]);
                        println!("payload length: {0}", f.length);
                    }

                    // read payload now we have length
                    let mut pl = vec![0u8; f.length.into()];
                    let _ = stream
                        .read_exact(&mut pl)
                        .expect("could not read payload from buffer");

                    match f.payload {
                        Some(p) => {
                            println!("received message: {0}", String::from_utf8_lossy(&p));
                        }
                        None => {
                            println!("no message received");
                        }
                    };
                    let msg = "hello mike";
                    let mut result = vec![0u8; 2 + msg.len()];
                    // first byte
                    // 1000 0001
                    let mut b: u8 = OpCode::to_u8(&OpCode::TEXT);
                    if f.is_final {
                        b |= 1 << 7
                    }
                    result[0] = b;

                    println!("final val is: {0}", result[0]);

                    let b1: u8 = 0;
                    result[1] = b1 | usize::to_ne_bytes(msg.len())[0];
                    result[2..].copy_from_slice(msg.as_bytes());

                    let mut w = BufWriter::new(&mut stream);
                    w.write(&result).unwrap();
                    w.flush().unwrap();
                }
                OpCode::CLOSE => {}
                OpCode::PING => {}
                OpCode::PONG => {}
            }
        }
    }
}

fn generate_hash(key: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input(key.to_owned() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    return base64::encode(hasher.result());
}

fn find_websocket_key(request: Vec<String>) -> Option<String> {
    let key = "Sec-WebSocket-Key:";
    for h in request.iter() {
        if h.contains(key) {
            let val = &h[key.len()..].trim_start();
            return Some(val.to_string());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::server::{find_websocket_key, generate_hash};

    #[test]
    fn generate_accept_hash() {
        assert_eq!(
            generate_hash("dGhlIHNhbXBsZSBub25jZQ==".to_string()),
            "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
        )
    }

    #[test]
    fn find_websocket_key_test() {
        let request = vec![
            "GET /ws HTTP/1.1".to_string(),
            "b".into(),
            "Sec-WebSocket-Key: abcd".into(),
        ];
        assert_eq!(find_websocket_key(request).unwrap(), "abcd");
    }
}
