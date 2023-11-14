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
            Ok(_stream) => {
                
                handle_connection(_stream)
        
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
      
       
        
    }

}


fn handle_connection(mut stream: TcpStream){
    let mut reader = BufReader::new(&mut stream);
    
    let mut request = String::new();
    match reader.read_line(&mut request) {
        Ok(bytes_read) => println!("{} bytes read", bytes_read),
        Err(error) => {
            println!("error reading from stream: {}", error);
        }
    }

    match stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
        Ok(bytes_written) => println!("{} bytes written", bytes_written),
        Err(error) => {
            println!("error writing to stream: {}", error);
        }
    };
}
