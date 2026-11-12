use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::rfc862::Rfc862Error;

// This function starts a TCP server that listens on the specified address and port.

const DEFAULT_ADDRESS: &str = "localhost";
const DEFAULT_PORT: u16 = 7007;
const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

fn get_full_addr() -> String {
    let full_addr = format!("{}:{}", DEFAULT_ADDRESS, DEFAULT_PORT);
    full_addr
}

fn read_from_stream(stream: &mut TcpStream) -> Result<[u8; DEFAULT_BUFFER_SIZE], Rfc862Error> {
    let mut buffer = [0; DEFAULT_BUFFER_SIZE];
    loop {
        let n = stream.read(&mut buffer).map_err(Rfc862Error::IoError)?;

        println!("server bytes read: {:?}", n);

        if n == 0 {
            return Err(Rfc862Error::ConnectionClosed);
        }

        let mut buffer = buffer.to_vec();

        buffer.truncate(n);

        write_to_stream(stream, buffer)?;
    }
}

fn write_to_stream(stream: &mut TcpStream, buffer: Vec<u8>) -> Result<(), Rfc862Error> {
    let _n = stream.write(&buffer).map_err(Rfc862Error::IoError)?;

    Ok(())
}

pub fn serve() -> Result<(), Rfc862Error> {
    let full_addr = get_full_addr();
    let listener = TcpListener::bind(&full_addr).map_err(Rfc862Error::IoError)?;

    println!("tcp listening on: {}", full_addr);

    loop {
        let (mut stream, addr) = listener.accept().map_err(Rfc862Error::IoError)?;

        println!("new client: {:?}", addr);
        thread::spawn(move || {
            read_from_stream(&mut stream).unwrap();
        });
    }
}

pub fn client() -> Result<(), Rfc862Error> {
    let server_addr = get_full_addr();

    let mut stream = TcpStream::connect(&server_addr).map_err(Rfc862Error::IoError)?;
    println!("connected");

    let mut buffer = [0; DEFAULT_BUFFER_SIZE];
    loop {
        let mut stdinbuffer = String::new();
        let _ = std::io::stdin().read_line(&mut stdinbuffer);
        let _ = stream.write_all(stdinbuffer.as_bytes());

        let n = stream.read(&mut buffer).map_err(Rfc862Error::IoError)?;

        let mut buffer = buffer.to_vec();
        buffer.truncate(n);

        println!("clients bytes read: {:?}", n);

        if n == 0 {
            return Err(Rfc862Error::ConnectionClosed);
        }

        let echo_msg = String::from_utf8(buffer).map_err(Rfc862Error::FromUtf8Error)?;
        println!("messgae: {:?}", echo_msg);
    }
}
