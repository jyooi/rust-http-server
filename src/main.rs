// Uncomment this block to pass the first stage

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => handle_connection(_stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);

    let mut request = String::new();
    match reader.read_line(&mut request) {
        Ok(bytes_read) => {
            println!("{} bytes read", bytes_read);
            let request_parts: Vec<&str> = request.split_whitespace().collect();
            // let http_method = request_parts.get(0).unwrap_or(&"");
            let http_path = request_parts.get(1).unwrap_or(&"");
            if http_path.to_string() == "/" {
                match stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
                    Ok(_bytes_written) => println!("HTTP 200 OK"),
                    Err(error) => {
                        println!("error writing to stream: {}", error);
                    }
                };
            } else {
                match stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n") {
                    Ok(_bytes_written) =>println!("HTTP 404 Not found"),
                    Err(error) => {
                        println!("error writing to stream: {}", error);
                    }
                };
            }
        }
        Err(error) => {
            println!("error reading from stream: {}", error);
        }
    }
}
