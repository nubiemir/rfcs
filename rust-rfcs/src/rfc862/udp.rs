use std::net::{SocketAddr, UdpSocket};

use crate::rfc862::Rfc862Error;

// This function starts a TCP server that listens on the specified address and port.

const DEFAULT_ADDRESS: &str = "localhost";
const DEFAULT_PORT: u16 = 7008;
const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

fn get_full_addr() -> String {
    let full_addr = format!("{}:{}", DEFAULT_ADDRESS, DEFAULT_PORT);
    full_addr
}

fn echo(buf: &[u8], socket: &UdpSocket, src: &SocketAddr) -> Result<(), Rfc862Error> {
    socket.send_to(buf, src).map_err(Rfc862Error::IoError)?;

    Ok(())
}

pub fn serve() -> Result<(), Rfc862Error> {
    let full_addr = get_full_addr();
    let mut buffer = [0; DEFAULT_BUFFER_SIZE];
    let socket = UdpSocket::bind(&full_addr).map_err(Rfc862Error::IoError)?;

    println!("udp listening on: {}", full_addr);

    loop {
        let (size, address) = socket
            .recv_from(&mut buffer)
            .map_err(Rfc862Error::IoError)?;

        println!("new client: {:?}", address);

        echo(&buffer[..size], &socket, &address).unwrap();
    }
}

pub fn client(lp: u8) -> Result<(), Rfc862Error> {
    let server_addr = get_full_addr();

    let client_addr = format!("localhost:800{}", lp);

    let socket = UdpSocket::bind(&client_addr).map_err(Rfc862Error::IoError)?;
    println!("udp connected");

    loop {
        let mut buffer = [0; DEFAULT_BUFFER_SIZE];
        let mut stdinbuffer = String::new();
        let _ = std::io::stdin().read_line(&mut stdinbuffer);
        let _ = socket
            .send_to(stdinbuffer.as_bytes(), &server_addr)
            .map_err(Rfc862Error::IoError)?;

        let (size, address) = socket
            .recv_from(&mut buffer)
            .map_err(Rfc862Error::IoError)?;
        let mut buffer = buffer.to_vec();
        buffer.truncate(size);

        let echo_msg = String::from_utf8(buffer).map_err(Rfc862Error::FromUtf8Error)?;
        println!("received from {}: {:?}", address, echo_msg);
    }
}
