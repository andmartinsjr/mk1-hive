use std::collections::VecDeque;
use std::io::Write;
use std::net::TcpStream;

use shared_data::DATA_COLLECTOR_ADDRESS;

use crate::errors::CollectorError;

/*
pub fn send_command(bytes: &[u8]) -> Result<(), CollectorError> {
    let mut stream =
        TcpStream::connect(DATA_COLLECTOR_ADDRESS).map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(bytes)
        .map_err(|_| CollectorError::UnableToConnect)?;
    Ok(())
}
*/

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    // Connect
    let mut stream =
        TcpStream::connect(DATA_COLLECTOR_ADDRESS).map_err(|_| CollectorError::UnableToConnect)?;

    // Send every queue item
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSendData);
        }
    }

    Ok(())
}
