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
            let http_path = request_parts.get(1).unwrap_or(&"").to_string();
            let params = http_path.splitn(3, '/').last();

           let body = match http_path.as_str() {
            "/" =>  format!("HTTP/1.1 200 OK\r\n\r\n"),
            path if path.starts_with("/echo/") => match params {
                Some(params) => {                    
                    let content = params.to_string();
                    print!("content {}",content);
                    format!("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\n\r\nContent-Length: {}\r\n\r\n{}", content.len(), content)
                },
                None => {
                    String::from("HTTP/1.1 404 Not Found\r\n\r\n")
                }
            },
            _ =>  format!("HTTP/1.1 404 Not Found\r\n\r\n"),
        };
           
           
             match stream.write(body.as_bytes()) {
                Ok(_bytes_written) => println!("HTTP 200 OK"),
                Err(error) => {
                    println!("error writing to stream: {}", error);
                }
            };
        }
        Err(error) => {
            println!("error reading from stream: {}", error);
        }
    }
}
