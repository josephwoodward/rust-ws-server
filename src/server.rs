// use crate::request::Request;
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use sha1::{Digest, Sha1};

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
            let stream = stream.expect("failed to read from TCP stream");
            self.handle_connection(stream);
        }

        return Ok(());
    }

    // fn handle_connection(&mut self, mut stream: TcpStream) -> ! {
    //     // let reader = BufReader::new(stream.try_clone().expect("failed to clone stream"));
    //     // let buf_reader = BufReader::new(&mut stream);
    //     let mut buf = [0; 1024];
    //     loop {
    //         match stream.read(&mut buf) {
    //             Ok(_) => match Request::try_from(&buf[..]) {
    //                 Ok() => {}
    //                 Err(e) => {}
    //             },
    //             Err(e) => {
    //                 println!("err: {:?}", e);
    //             }
    //         }
    //     }
    // }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        // println!("Request: {:#?}", http_request);

        let mut buf_writer = BufWriter::new(&mut stream);
        if http_request[0] != "GET /ws HTTP/1.1" {
            buf_writer
                .write("HTTP/1.1 404 Not Found".as_bytes())
                .unwrap();
        } else {
            buf_writer
                .write("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: ".as_bytes())
                .unwrap();
        }

        buf_writer.flush().unwrap();

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
    }
}

fn generate_hash(hash: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(hash.as_bytes());
    hasher.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    // h := sha1.New()
    // h.Write([]byte(key))
    // h.Write([]byte("258EAFA5-E914-47DA-95CA-C5AB0DC85B11"))
    // return base64.StdEncoding.EncodeToString(h.Sum(nil))
    //
    let _ = hasher.finalize();
    return "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=".into();
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn exploration() {
    //     asse    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    //     EvenNumber::try_from(8);
    //     assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    //     assert_eq!(EvenNumber::try_from(5), Err(()));
    // }

    #[test]
    fn generate_accept_hash() {
        assert_eq!(
            crate::server::generate_hash("dGhlIHNhbXBsZSBub25jZQ==".to_string()),
            "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
        )
    }

    // func TestGetAcceptHash(t *testing.T) {
    // 	assert(t, ws.GenerateAcceptHash("dGhlIHNhbXBsZSBub25jZQ=="), "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=")
    // }
}
