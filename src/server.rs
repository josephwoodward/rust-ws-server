use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("server running and listening on: {}", &self.addr);

        for stream in listener.incoming() {
            let stream = stream.expect("failed to read from TCP stream");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) -> ! {
        let mut buf = [0 as u8; 1024];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        continue;
                    }

                    println!("buffer lenght: {}", len);
                }
                Err(e) => {
                    println!("err: {:?}", e);
                }
            }
        }
    }

    // fn handle_connection(&mut self, mut stream: TcpStream) {
    //     let buf_reader = BufReader::new(&mut stream);
    //     let http_request: Vec<_> = buf_reader
    //         .lines()
    //         .map(|result| result.unwrap())
    //         .take_while(|line| !line.is_empty())
    //         .collect();

    //     println!("Request: {:#?}", http_request);
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
