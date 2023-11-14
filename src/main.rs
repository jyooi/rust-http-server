// Uncomment this block to pass the first stage
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
      
       
        let stream = stream.unwrap();
        handle_connection(stream)

    }

}


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0,255];

    stream.read(&mut buffer).unwrap();
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // print buffer

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}
