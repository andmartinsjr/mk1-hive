use std::io::Write;
use std::net::TcpStream;

use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS};

pub fn send_command(command: &CollectorCommandV1) {
    let bytes = shared_data::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = TcpStream::connect(DATA_COLLECTOR_ADDRESS).unwrap();
    stream.write_all(&bytes).unwrap();
}
