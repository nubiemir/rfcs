use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

const QUOTES: [&str; 20] = [
    "Stay hungry, stay foolish.",
    "Knowledge is power.",
    "Simplicity is the ultimate sophistication.",
    "The journey is the reward.",
    "Great things take time.",
    "Fortune favors the bold.",
    "Actions speak louder than words.",
    "Make it work, make it right, make it fast.",
    "First, solve the problem. Then, write the code.",
    "Programs must be written for people to read.",
    "Talk is cheap. Show me the code.",
    "It always seems impossible until it's done.",
    "Well done is better than well said.",
    "The secret of getting ahead is getting started.",
    "Without change there is no innovation.",
    "Dream big. Start small. Act now.",
    "A smooth sea never made a skilled sailor.",
    "Great things are done by a series of small things.",
    "Discipline is choosing what you want most.",
    "The best way out is always through.",
];

const SERVER_ADDR: &'static str = "localhost:8000";

#[derive(Debug)]
#[allow(unused)]
pub enum Rfc865Error {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Rfc865Error>;

fn generate_quote() -> &'static [u8] {
    let num = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 20;
    QUOTES[num as usize].as_bytes()
}

pub fn tcp_server() -> Result<()> {
    let listener = TcpListener::bind(SERVER_ADDR).map_err(Rfc865Error::IoError)?;

    loop {
        let (mut socket, _) = listener.accept().map_err(Rfc865Error::IoError)?;

        let quote = generate_quote();

        thread::spawn(move || match socket.write_all(quote) {
            Err(err) => {
                println!("something went wrong: {:?}", err);
            }
            _ => {}
        });
    }
}

pub fn udp_server() -> Result<()> {
    let socket = UdpSocket::bind(SERVER_ADDR).map_err(Rfc865Error::IoError)?;

    loop {
        let (_, addr) = socket
            .recv_from(&mut [0; 1])
            .map_err(Rfc865Error::IoError)?;

        let quote = generate_quote();

        socket.send_to(quote, addr).map_err(Rfc865Error::IoError)?;
    }
}

pub fn tcp_client() -> Result<()> {
    let mut stream = TcpStream::connect(SERVER_ADDR).map_err(Rfc865Error::IoError)?;

    let mut buffer = [0; 8 * 1024];

    loop {
        let n = stream.read(&mut buffer).map_err(Rfc865Error::IoError)?;

        if n == 0 {
            break;
        }

        let mut buffer = buffer.to_vec();
        buffer.truncate(n);

        let res = String::from_utf8(buffer).map_err(Rfc865Error::FromUtf8Error)?;

        println!("Quote of the day\n{}", res);
    }

    Ok(())
}

pub fn upd_client() -> Result<()> {
    let client_addr = format!("localhost:0");

    let client_socket = UdpSocket::bind(client_addr).map_err(Rfc865Error::IoError)?;

    client_socket
        .send_to(&[0; 0], SERVER_ADDR)
        .map_err(Rfc865Error::IoError)?;

    let mut buffer = [0; 8 * 1024];

    let (size, _) = client_socket
        .recv_from(&mut buffer)
        .map_err(Rfc865Error::IoError)?;

    let mut res = buffer.to_vec();
    res.truncate(size);

    let res = String::from_utf8(res).map_err(Rfc865Error::FromUtf8Error)?;

    println!("Quote of the day\n{}", res);

    Ok(())
}
