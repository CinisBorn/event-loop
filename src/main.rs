use event_loop::event_producer;

fn main() {
    let connection = event_producer::connect().unwrap();

    println!("Connection at: {}", connection.1);
    
    loop {
        let payload = event_producer::accept_connection(&connection).unwrap();
        println!("{}", String::from_utf8(payload).unwrap());
    }
}
