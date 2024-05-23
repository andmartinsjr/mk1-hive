use std::io::Write;
use std::net::TcpStream;

use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS};

use crate::errors::CollectorError;

pub fn send_command(command: &CollectorCommandV1) -> Result<(), CollectorError> {
    let bytes = shared_data::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream =
        TcpStream::connect(DATA_COLLECTOR_ADDRESS).map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(&bytes)
        .map_err(|_| CollectorError::UnableToConnect)?;
    Ok(())
}
