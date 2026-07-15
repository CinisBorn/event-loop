use event_loop::event_emitter;

fn main() {
    let connection = event_emitter::connect().unwrap();

    println!("Connection at: {}", connection.1);
    
    loop {
        let payload = event_emitter::accept_connection(&connection).unwrap();
        println!("{}", String::from_utf8(payload).unwrap());
    }
}
