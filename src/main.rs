#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "HTTP/1.1 200 OK",
            HttpStatus::NotFound => "HTTP/1.1 404 Not Found",
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("failed to read the line: {}", e);
                String::new()
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();

    let uri = http_request
        .first()
        .expect("http request is empty")
        .split_whitespace()
        .nth(1)
        .unwrap_or_default();

    let user_agent = http_request
        // i should check each line && look up USER-AGENT?
        .get(2)
        .expect("user agent not found")
        .split_whitespace()
        .nth(1)
        .unwrap_or_default();

    // if it starts witch /echo then process it
    // /echo/{str} --> return str and it length and type

    // in case of NotFound
    let mut length: i32 = -1;

    let (status_line, content) = match uri {
        "/" => (HttpStatus::Ok.as_str(), ""),
        "/user-agent" => {
            length = user_agent.len() as i32;
            (HttpStatus::Ok.as_str(), user_agent)
        }
        path if path.starts_with("/echo/") => {
            let content = path.strip_prefix("/echo/").unwrap_or("");
            (HttpStatus::Ok.as_str(), content)
        }
        _ => (HttpStatus::NotFound.as_str(), ""),
    };

    let response = format!(
        "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, content
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_connection(stream) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("could not write the buffer: {}", e);
                    return;
                }
            },
            Err(e) => eprintln!("error: {}", e),
        }
    }
}
