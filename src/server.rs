use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;
use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener},
};

use crate::request::Request;
use crate::router::Router;

pub struct Server {
    address: SocketAddr,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }

    pub fn handle_client(
        mut stream: TcpStream,
        router: &Router,
        ctx: Option<&HashMap<String, String>>,
    ) -> io::Result<usize> {
        // Buffer to store the data received from the client
        let mut buffer = [0; 512];

        // Read data from the stream
        match stream.read(&mut buffer) {
            Ok(_) => {
                // Convert buffer to a string and print the received data
                match str::from_utf8(&buffer) {
                    Ok(request) => {
                        println!("Received request:\n{}", request);
                        let request_lines: Vec<&str> = request.split("\r\n").collect();
                        let request = Request::from(request_lines);
                        let request_string: String = (&request).into();

                        println!("Request after parsing:\n{}", request_string);
                        dbg!(&request.method);

                        let response: String = router.handle(&request, ctx).into();
                        stream.write(response.as_bytes())
                    }
                    Err(_) => todo!(),
                }
            }
            Err(_) => todo!(),
        }
    }

    pub fn serve(&self, router: &Router, ctx: Option<&HashMap<String, String>>) -> io::Result<()> {
        let listener = TcpListener::bind(self.address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // thread::spawn(|| {
                    let _ = Server::handle_client(stream, router, ctx);
                    // });
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                }
            }
        }
        Ok(())
    }
}
