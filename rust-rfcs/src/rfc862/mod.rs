pub mod tcp;
pub mod udp;

#[derive(Debug)]
#[allow(unused)]
pub enum Rfc862Error {
    IoError(std::io::Error),
    ConnectionClosed,
    FromUtf8Error(std::string::FromUtf8Error),
}
