use std::net::TcpStream;
use std::io::Read; 

pub struct Worker();

impl Worker {
    fn read_to_string(mut stream: TcpStream) -> String {
        let mut buf = vec![0u8; 512];
        let read_bytes = stream.read(&mut buf).expect("bytes to be read");
    
        buf.truncate(read_bytes);
        
        String::from_utf8(buf).unwrap()
    }

    pub fn show_client_message(stream: TcpStream) {
        println!("Client said: {}", Self::read_to_string(stream));
    }
}