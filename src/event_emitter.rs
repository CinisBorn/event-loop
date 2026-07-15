use std::{io::Read, net};
use crate::error::EventEmitterErrors;

pub struct Connection(net::TcpListener, pub net::SocketAddr);

pub fn connect() -> Result<Connection, EventEmitterErrors> {
    let socket = net::TcpListener::bind("127.0.0.1:0")?;
    let address: net::SocketAddr = socket.local_addr()?;

    Ok(Connection(socket, address))
}

pub fn accept_connection(connection: &Connection) -> Result<Vec<u8>, EventEmitterErrors> {
    let socket = &connection.0;
    let mut buffer = vec![0; 512];
    
    let (mut stream, _) = socket.accept().expect("tcp connection failed");
    let bytes_read = stream.read(&mut buffer)?;

    buffer.truncate(bytes_read);
    
    Ok(buffer)
}