//! The even loop.
use crate::{error::DriverErrors, worker::Worker};
use std::net::{self, TcpStream};

/// The events support by the event loop.
#[derive(Debug)]
pub enum Events {
    /// This event is triggered whanever a new connection is accepted.
    Connected(TcpStream),
}

/// The event loop type. It does not hold any state. The purpose of type `Driver` is merely for
/// organization and scalability.
#[derive(Debug)]
pub struct Driver();

impl Driver {
    /// Starts the event loop openning a new connection in a random port.
    ///
    /// # Errors
    /// It will return a `DriverError::Io` if it's not possible to open a new connection.
    pub fn start() -> Result<(), DriverErrors> {
        let (listener, address) = connect()?;
        println!("Connect at port: {}", address);

        loop {
            let (stream, _) = listener.accept()?;
            Self::dispatch_event(Events::Connected(stream));
        }
    }

    pub fn dispatch_event(event: Events) {
        match event {
            Events::Connected(stream) => Worker::show_client_message(stream),
        }
    }
}

fn connect() -> Result<(net::TcpListener, net::SocketAddr), DriverErrors> {
    let socket = net::TcpListener::bind("127.0.0.1:0")?;
    let address: net::SocketAddr = socket.local_addr()?;

    Ok((socket, address))
}
