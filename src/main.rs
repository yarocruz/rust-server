use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

/*
The HTTP PROTOCOL is a simple test-based

The response must follow this format in text

HTTP-Version Status-Code Reason-Phrase CRLF (Carriage Return Line Feed)
headers CRLF
message-body

Literally this for a OK 200
HTTP/1.1 200 OK\r\n\r\n
*/

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

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    // convert the response into bytes and write it all to the stream
    // the write_all method here is completing the exchange. after this you stop seeing an error in the browser and see a blank page
    stream.write_all(response.as_bytes()).unwrap();

    //println!("Request: {:#?}", http_request);
}
