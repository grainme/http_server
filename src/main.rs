#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

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

    // TODO: Improve this implementation
    let uri = http_request[0].split_whitespace().collect::<Vec<_>>()[1];

    let status_line = match uri {
        "/" => "HTTP/1.1 200 OK",
        _ => "HTTP/1.1 404 Not Found",
    };

    let http_response = format!("{status_line}\r\n\r\n");
    stream.write_all(http_response.as_bytes())?;
    Ok(())
}

fn main() {
    println!("Logs from your program will appear here!");

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
