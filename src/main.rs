use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // TcpListener.bind() creates a "listener" at the address and port indicated and returns a Result type
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for every incoming request to the listener create a stream
    for stream in listener.incoming() {
        // shadow the stream hear. we're not doing anything with the stream here for now
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// function will handle the streams
fn handle_connection(mut stream: TcpStream) {
    /*
        Every time a request is done we're going to read that stream into a buffer
        and collect those reads/treams into a vector(growable array for the javascripties)
     */
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())// I believe we need to use this to take ownership
    .collect();

    println!("Request: {:#?}", http_request);
}
