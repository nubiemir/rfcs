use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    thread,
    time::SystemTime,
};

const SERVER_ADDR: &str = "localhost:8000";

const WEEK_DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const EPOCH_OFFSET: u128 = 719468;
const DAYS_ERA: u128 = 146097;
const DAYS_MS: u128 = 86400000;
const HOURS_MS: u128 = 3600000;
const MIN_MS: u128 = 60000;
const WEEK_EPOCH_OFFSET: u128 = 3;

#[derive(Debug)]
#[allow(unused)]
pub enum Rfc867Error {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Rfc867Error>;

fn civil_from_days(days: u128) -> (u128, u128, u128) {
    let z = days + EPOCH_OFFSET;
    let era = z / DAYS_ERA;
    let doe = z - era * DAYS_ERA;
    let yoe = (doe - (doe / 1460) + (doe / 36524) - (doe / 146096)) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = year + u128::from(month <= 2);
    let day = doy - (153 * mp + 2) / 5 + 1;
    (year, month, day)
}

fn date_generator() -> String {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let days = timestamp / DAYS_MS;
    let days_rem = timestamp % DAYS_MS;
    let hours = days_rem / HOURS_MS;
    let hours_rem = days_rem % HOURS_MS;
    let minutes = hours_rem / MIN_MS;
    let min_rem = hours_rem % MIN_MS;
    let secs = min_rem / 1000;
    let milli = min_rem % 1000;

    let (year, month, day) = civil_from_days(days);
    let month = MONTHS[(month - 1) as usize];
    let week = WEEK_DAYS[((days + WEEK_EPOCH_OFFSET) % 7) as usize];
    let date = format!(
        "{}, {} {}, {} {}:{}:{}:{}-UTC",
        week, year, month, day, hours, minutes, secs, milli
    );
    date
}

pub fn tcp_server() -> Result<()> {
    let listener = TcpListener::bind(SERVER_ADDR).map_err(Rfc867Error::IoError)?;

    loop {
        let (mut socket, _) = listener.accept().map_err(Rfc867Error::IoError)?;

        let date = date_generator();

        thread::spawn(move || match socket.write_all(date.as_bytes()) {
            Err(err) => {
                println!("something went wrong: {:?}", err);
            }
            _ => {}
        });
    }
}

pub fn udp_server() -> Result<()> {
    let socket = UdpSocket::bind(SERVER_ADDR).map_err(Rfc867Error::IoError)?;

    loop {
        let (_, addr) = socket
            .recv_from(&mut [0; 1])
            .map_err(Rfc867Error::IoError)?;

        let date = date_generator();

        socket
            .send_to(date.as_bytes(), addr)
            .map_err(Rfc867Error::IoError)?;
    }
}

pub fn tcp_client() -> Result<()> {
    let mut stream = TcpStream::connect(SERVER_ADDR).map_err(Rfc867Error::IoError)?;

    let mut buffer = [0; 8 * 1024];

    loop {
        let n = stream.read(&mut buffer).map_err(Rfc867Error::IoError)?;

        if n == 0 {
            break;
        }

        let mut buffer = buffer.to_vec();
        buffer.truncate(n);

        let res = String::from_utf8(buffer).map_err(Rfc867Error::FromUtf8Error)?;

        println!("{}", res);
    }

    Ok(())
}

pub fn udp_client() -> Result<()> {
    let client_addr = format!("localhost:0");

    let client_socket = UdpSocket::bind(client_addr).map_err(Rfc867Error::IoError)?;

    client_socket
        .send_to(&[0; 0], SERVER_ADDR)
        .map_err(Rfc867Error::IoError)?;

    let mut buffer = [0; 8 * 1024];

    let (size, _) = client_socket
        .recv_from(&mut buffer)
        .map_err(Rfc867Error::IoError)?;

    let mut res = buffer.to_vec();
    res.truncate(size);

    let res = String::from_utf8(res).map_err(Rfc867Error::FromUtf8Error)?;

    println!("{}", res);

    Ok(())
}
