use std::net::TcpListener;
fn main() {
    // TcpListener.bind() creates a "listener" at the address and port indicated and returns a Result type
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for every incoming request to the listener create a stream
    for stream in listener.incoming() {
        // shadow the stream hear. we're not doing anything with the stream here for now
        let stream = stream.unwrap();
        println!("Connection established");
    }
}
