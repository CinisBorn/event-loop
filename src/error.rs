use std::io;

#[derive(Debug)]
pub enum EventEmitterErrors {
    Io(io::Error)
}

impl From<io::Error> for EventEmitterErrors {
    fn from(value: io::Error) -> Self {
        self::EventEmitterErrors::Io(value)
    }
}