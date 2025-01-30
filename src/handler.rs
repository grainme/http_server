use crate::response::HttpStatus;
use std::error::Error;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::sleep;

pub async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request = Vec::new();

    let mut line = String::new();
    while buf_reader.read_line(&mut line).await? > 0 {
        if line == "\r\n" {
            break;
        }
        request.push(line.clone());
        line.clear();
    }

    let uri = request
        .iter()
        .find(|line| line.to_ascii_lowercase().starts_with("get"))
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("");

    let user_agent = request
        .iter()
        .find(|line| line.to_ascii_lowercase().starts_with("user-agent"))
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("");

    let (status_line, content) = match uri {
        "/" => (HttpStatus::Ok.as_str(), ""),
        "/user-agent" => {
            println!("trace test-agent: {}", user_agent);
            sleep(Duration::from_secs(2)).await;
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
        status_line,
        content.len(),
        content
    );

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
