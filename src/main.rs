use std::{
    env, fs,
    fs::File,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let args: Vec<String> = env::args().collect();
    let mut buf_reader = BufReader::new(&mut stream);

    let buffer = buf_reader.fill_buf().unwrap();
    // println!("buffer: {:?}", buffer);
    let result = String::from_utf8(buffer.to_vec()).unwrap();

    // println!("result: {:?}", result);
    // println!("buf_reader: {:?}", buf_reader);
    let request_line: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("request_line: {:?}", request_line);
    let user_agent_header = "User-Agent:";
    let user_agent = request_line
        .iter()
        .find(|line| line.starts_with(user_agent_header))
        .map(|line| line.trim_start_matches(user_agent_header).trim())
        .unwrap_or_else(|| "Unknown User Agent");

    let split_line: Vec<_> = request_line[0].split(" ").collect();
    let method = split_line[0];
    let mut path = split_line[1].to_string();
    if method == "GET" && path == "/" {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
    if method == "GET" && path.starts_with("/echo/") {
        path = path.replace("/echo", "");
        if path.starts_with("/") {
            path.remove(0);
        }
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            path.len(),
            path
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else if method == "GET" && path.starts_with("/user-agent") {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent.len(),
            user_agent
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else if method == "GET" && path.starts_with("/files") {
        let dir = args[2].to_string();
        path = path.replace("/files", "");
        if path.starts_with("/") {
            path.remove(0);
        }
        // println!("{} {}", path, filename);
        let full_path = format!("{dir}/{path}");
        if Path::new(&full_path).exists() {
            let contents = fs::read_to_string(&full_path);
            match contents {
                Ok(contents) => {
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{contents}",
                                    contents.len(),
                                );
                    stream.write_all(response.as_bytes()).unwrap();
                }
                Err(..) => {
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        } else {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    } else if method == "POST" && path.starts_with("/files") {
        let dir = args[2].to_string();
        path = path.replace("/files", "");
        if path.starts_with("/") {
            path.remove(0);
        }
        println!("{}", path);
        // println!("{} {}", path, filename);
        let full_path = format!("{dir}/{path}");
        let body_header = "\r\n\r\n";
        let body: String = result
            .splitn(2, body_header)
            .nth(1)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "No Body".to_string());
        if body == "No Body" {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let mut file = File::create(full_path).unwrap();
            file.write_all(body.as_bytes()).unwrap();
            let response = format!("HTTP/1.1 201 OK \r\n\r\n");
            stream.write_all(response.as_bytes()).unwrap();
        }
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}