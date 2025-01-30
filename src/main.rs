#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use http_server::ThreadPool;

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

/// Handling different http endpoints.
///
/// /echo/{str} this would return the string, its length and its type.
/// /user-agent would return user-agent as response body
///
///
/// # Examples
///
fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&stream);
    let request: Vec<String> = buf_reader
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

    // i should check each line && look up URI
    let uri = match request
        .iter()
        .position(|line| line.to_ascii_lowercase().starts_with("get"))
    {
        Some(ind) => request
            .get(ind)
            .map_or("", |v| v)
            .split_whitespace()
            .nth(1)
            .unwrap(),
        None => "",
    };

    // i should check each line && look up USER-AGENT
    let user_agent = match request
        .iter()
        .position(|line| line.to_ascii_lowercase().starts_with("user-agent"))
    {
        Some(ind) => request
            .get(ind)
            .map_or("", |v| v)
            .split_whitespace()
            .nth(1)
            .unwrap(),
        None => "",
    };

    // if it starts witch /echo then process it
    // /echo/{str} --> return str and it length and type

    let mut length: i32 = 0;

    let (status_line, content) = match uri {
        "/" => (HttpStatus::Ok.as_str(), ""),
        "/user-agent" => {
            length = user_agent.len() as i32;
            println!("trace test-agent: {}", user_agent);
            thread::sleep(Duration::from_secs(2));
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
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        pool.execute(|| match stream {
            Ok(stream) => match handle_connection(stream) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("could not write the buffer: {}", e);
                    return;
                }
            },
            Err(e) => eprintln!("error: {}", e),
        });
    }
    println!("Shutting down");
}
