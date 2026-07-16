use std::io;

#[derive(Debug)]
pub enum DriverErrors {
    Io(io::Error),
}

impl From<io::Error> for DriverErrors {
    fn from(value: io::Error) -> Self {
        self::DriverErrors::Io(value)
    }
}
