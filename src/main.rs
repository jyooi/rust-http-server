// Uncomment this block to pass the first stage

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, collections::HashMap,
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
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content.len(), content)
                },
                None => {
                    String::from("HTTP/1.1 404 Not Found\r\n\r\n")
                }
            },
            path if path.starts_with("/user-agent") => match params {
                Some(params) => {          
                    let mut headers = HashMap::new();
                        loop {
                            let mut line = String::new();
                            match reader.read_line(&mut line) {
                                Ok(bytes_read) => {
                                    if bytes_read == 0 || line == "\r\n" {
                                        break;
                                    }
                                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                                    if parts.len() == 2 {
                                        headers.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                                    }
                                },
                                Err(error) => {
                                    println!("error reading from stream: {}", error);
                                    return;
                                }
                            };
                        }
                    let default_agent = "Unknown User-Agent".to_string();
                    let user_agent = headers.get("User-Agent").unwrap_or(&default_agent);
                    let content = params.to_string();
                    print!("content {}",content);
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content.len(), user_agent)
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
