// Uncomment this block to pass the first stage

use std::{
    io::{BufRead, BufReader, Write, Read},
    net::{TcpListener, TcpStream}, collections::HashMap,
};
use std::env;
use std::fs;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut directory = "default_directory";
    for i in 0..args.len() {
        if args[i] == "--directory" && i + 1 < args.len() {
            directory = &args[i + 1];
            break;
        }
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let directory = directory.to_string();

        thread::spawn(move || { // Spawn a new thread for each connection
            handle_connection(stream, directory);
        });
    }
}
fn handle_connection(mut stream: TcpStream, directory: String) {
    let mut reader = BufReader::new(&mut stream);

    let mut request = String::new();
    match reader.read_line(&mut request) {
        Ok(bytes_read) => {
            println!("{} bytes read", bytes_read);
            let request_parts: Vec<&str> = request.split_whitespace().collect();
            let http_path = request_parts.get(1).unwrap_or(&"").to_string();
            let params = http_path.splitn(3, '/').last();
            let request_lines: Vec<&str> = request.split('\n').collect();
            let request_line = request_lines[0];
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            let method = parts[0];
           let body = match http_path.as_str() {
            "/" =>  format!("HTTP/1.1 200 OK\r\n\r\n"),
            path if path.starts_with("/echo/") => match params {
                Some(params) => {                    
                    let content = params.to_string();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content.len(), content)
                },
                None => {
                    String::from("HTTP/1.1 404 Not Found\r\n\r\n")
                }
            },
            path if path.starts_with("/user-agent") => match params {
                Some(_params) => {          
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
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent)
                },
                None => {
                    String::from("HTTP/1.1 404 Not Found\r\n\r\n")
                }
            },
            path if path.starts_with("/files/") => {
                match method {
                    "POST" => {
                        let filename = &path[7..];
                        let filepath = format!("{}/{}", directory, filename);
                        let mut file = std::fs::File::create(filepath).unwrap();
                        let mut body = Vec::new();
                        reader.read_to_end(&mut body).unwrap();
                        file.write_all(&body).unwrap();
                        format!("HTTP/1.1 201 Created\r\n\r\n")
                    },
                    "GET" => {
                        let filename = &path[7..];
                        let filepath = format!("{}/{}", directory, filename);
                        match fs::read(&filepath) {
                            Ok(contents) => {
                                format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", contents.len(), String::from_utf8_lossy(&contents))
                            },
                            Err(_) => {
                                String::from("HTTP/1.1 404 Not Found\r\n\r\n")
                            }
                        }
                    },
                    _ =>  format!("HTTP/1.1 405 Method Not Allowed\r\n\r\n"),
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
