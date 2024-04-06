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
    masking_key: Option<[u8; 4]>,
    payload_length: u8,
    payload: Option<Vec<u8>>,
}

impl Frame {
    pub fn new(head: [u8; 2]) -> Self {
        Self {
            op_code: OpCode::from_u8(head[0]),
            is_final: (head[0] & 0x80) == 0x00,
            is_masked: (head[1] & 0x80) == 0x80,
            payload_length: head[1] & 0x7F,
            masking_key: None,
            payload: None,
        }

        // println!("payload is masked: {0}", is_masked);
        // println!("payload is final: {0}", is_final);
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

    fn upgrade_connection(&mut self, mut stream: TcpStream) -> ! {
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
            let mut head = [0u8; 2];

            // TODO: Improve error handling of reading into the buffer
            let s = stream.read(&mut head).expect("failed to read from buffer");
            if s == 2 {
                println!("read {0} bytes from stream for head", s);
            }

            // let mask = if masked {
            //     Some(self.buffer.get_u32().to_be_bytes())
            // } else {
            //     None
            // };

            let mut f = Frame::new(head);
            println!("payload is masked: {0}", f.is_masked);

            match f.op_code {
                OpCode::TEXT => {
                    if f.payload_length > 0 {
                        f.payload = Some(vec![0; f.payload_length.into()]);
                        println!("payload length: {0}", f.payload_length);
                    }

                    // println!("received payload {0}", payload.len());
                    if f.is_masked {
                        let mut masking_key = [0u8; 4];
                        let s = stream
                            .read(&mut masking_key)
                            .expect("could not read masking key from stream");
                        println!("masking key bytes read: {0}", s);

                        f.masking_key = Some(masking_key);
                        // let s = match String::from_utf8(masking_key.to_vec()) {
                        //     Ok(v) => v,
                        //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        // };

                        // read payload now we have length
                        let mut payload = vec![0u8; f.payload_length.into()];
                        let n = stream
                            .read(&mut payload)
                            .expect("could not read payload from stream");

                        println!("received payload {0}, actual {1}", n, payload.len());
                        // println!("received payload {0}", payload.len());

                        // if let Some(payload) = f.payload {
                        //     let s = match String::from_utf8(payload) {
                        //         Ok(v) => v,
                        //         Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        //     };
                        // }

                        // for i := uint64(0); i < frame.Length; i++ {
                        // 	payload[i] ^= frame.MaskingKey[i%4]
                        // }
                        // let s = match String::from_utf8(f.payload) {
                        //     Ok(v) => v,
                        //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        // };

                        // let s = match String::from_utf8(masking_key.to_vec()) {
                        //     Ok(v) => v,
                        //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        // };

                        println!("result: {}", s);
                        match f.payload {
                            Some(mut pl) => {
                                unmask_fallback(&mut pl, masking_key);

                                let _ = match String::from_utf8(payload) {
                                    Ok(v) => println!("received message: {0}", v),
                                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                };
                            }
                            None => (),
                        }
                    }

                    // match f.payload {
                    //     Some(p) => {
                    //         println!("received message: {0}", String::from_utf8_lossy(&p));
                    //     }
                    //     None => {
                    //         println!("no message received");
                    //     }
                    // };
                    let msg = "hello mike";
                    let mut result = vec![0u8; 2 + msg.len()];
                    // first byte
                    // 1000 0001
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

fn unmask_easy(payload: &mut [u8], mask: [u8; 4]) {
    for i in 0..payload.len() {
        payload[i] ^= mask[i & 3];
    }
}

fn unmask_fallback(buf: &mut [u8], mask: [u8; 4]) {
    let mask_u32 = u32::from_ne_bytes(mask);

    let (prefix, words, suffix) = unsafe { buf.align_to_mut::<u32>() };
    unmask_easy(prefix, mask);
    let head = prefix.len() & 3;
    let mask_u32 = if head > 0 {
        if cfg!(target_endian = "big") {
            mask_u32.rotate_left(8 * head as u32)
        } else {
            mask_u32.rotate_right(8 * head as u32)
        }
    } else {
        mask_u32
    };
    for word in words.iter_mut() {
        *word ^= mask_u32;
    }
    unmask_easy(suffix, mask_u32.to_ne_bytes());
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
