use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_server::{ThreadPool, Todo};

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
    let pool = ThreadPool::new(4);

    // for every incoming request to the listener create a stream
    for stream in listener.incoming() {
        // shadow the stream hear. we're not doing anything with the stream here for now
        let stream = stream.unwrap();

        // create or spawn a thread
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

// function will handle the streams
fn handle_connection(mut stream: TcpStream) {
    /*
        Every time a request is done we're going to read that stream into a buffer
        and collect those reads/treams into a vector(growable array for the javascripties)
     */
    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap_or_else(|| Ok("GET / HTTP/1.1".to_string())).unwrap();

    // another way of create the buffer
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Let's make sure to only send response and html if it's a GET to root /
    // let (status_line, filename) = match buffer {
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
    //     "GET /click HTTP/1.1" => ("HTTP/1.1 200 OK", "click.html"),
    //     "GET /sleep HTTP/1.1" => { 
    //         thread::sleep(Duration::from_secs(5));
    //         ("HTTP/1.1 200 OK", "click.html") 
    //     }
    //     _ => ("HTTP/1.1 404 NOT FOUND", "404.html") 
    // };
    
    // let contents = fs::read_to_string(filename).unwrap();
    // let length = contents.len();

    // let response = format!(
    //     "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    // );

    let todos: Vec<Todo> = vec![
        Todo::new(1, "Taste htmx".to_string(), true),
        Todo::new(2, "Buy a unicorn".to_string(), false),
    ];

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let click = b"GET /click HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        println!("These are the todos {:?}", todos);
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(click) {
        ("HTTP/1.1 200 OK", "<h1>Todo</h1") 
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = if filename.chars().nth(0) == Some('<') {
        filename.to_string()
    } else {
        fs::read_to_string(filename).unwrap()
    };


    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    //println!("Request: {:#?}", http_request);
}
