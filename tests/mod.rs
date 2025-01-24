use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

struct TestServer {
    child: Child,
}

impl TestServer {
    fn start() -> io::Result<TestServer> {
        let child = Command::new("./target/debug/http-server").spawn()?;
        thread::sleep(Duration::from_millis(100));
        Ok(TestServer { child })
    }

    fn send_request(&self, request: &str) -> io::Result<String> {
        let mut stream = TcpStream::connect("127.0.0.1:4221")?;
        stream.write_all(request.as_bytes())?;
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        Ok(response)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

// Stage 1: Basic server functionality
#[test]
fn test_server_starts_and_binds() {
    let _server = TestServer::start().expect("Failed to start server");
    assert!(TcpStream::connect("127.0.0.1:4221").is_ok());
    assert!(
        TcpStream::connect("127.0.0.1:4222").is_err(),
        "Server should only bind to port 4221"
    );
}

// Stage 2: Basic HTTP response
#[test]
fn test_http_responses() {
    let server = TestServer::start().expect("Failed to start server");

    // Test root path (200)
    let response = server
        .send_request("GET / HTTP/1.1\r\n\r\n")
        .expect("Failed to get response");
    assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(response.contains("\r\n\r\n"));

    // Test other paths (404)
    let response = server
        .send_request("GET /unknown HTTP/1.1\r\n\r\n")
        .expect("Failed to get response");
    assert!(response.contains("404 Not Found"));
}

// Stage 3: Path handling
#[test]
fn test_path_handling() {
    let server = TestServer::start().expect("Failed to start server");

    // Test various paths
    let test_cases = [
        ("GET / HTTP/1.1\r\n\r\n", "200 OK"),
        ("GET /test HTTP/1.1\r\n\r\n", "404"),
        ("GET /test/nested HTTP/1.1\r\n\r\n", "404"),
        ("GET /test?param=value HTTP/1.1\r\n\r\n", "404"),
    ];

    for (request, expected) in test_cases {
        let response = server
            .send_request(request)
            .expect("Failed to get response");
        assert!(
            response.contains(expected),
            "Failed for request: {}",
            request
        );
    }
}

// Stage 4: User-Agent handling
#[test]
fn test_user_agent() {
    let server = TestServer::start().expect("Failed to start server");

    // Test basic user-agent
    let response = server
        .send_request(
            "GET /user-agent HTTP/1.1\r\n\
             Host: localhost\r\n\
             User-Agent: foobar/1.2.3\r\n\
             Accept: */*\r\n\r\n",
        )
        .expect("Failed to get response");

    dbg!(&response);
    assert!(response.contains("HTTP/1.1 200 OK\r\n"));
    assert!(response.contains("Content-Type: text/plain\r\n"));
    assert!(response.contains("Content-Length: 12\r\n"));
    assert!(response.ends_with("foobar/1.2.3"));

    // Test case insensitivity
    let response = server
        .send_request("GET /user-agent HTTP/1.1\r\nUSER-AGENT: test-agent\r\n\r\n")
        .expect("Failed to get response");
    assert!(response.ends_with("test-agent"));

    // Test missing user-agent
    let response = server
        .send_request("GET /user-agent HTTP/1.1\r\n\r\n")
        .expect("Failed to get response");
    assert!(response.contains("Content-Length: 0\r\n"));
}

// Error handling
#[test]
fn test_error_cases() {
    let server = TestServer::start().expect("Failed to start server");

    let test_cases = [
        ("\r\n\r\n", "response not empty"),
        ("INVALID / HTTP/1.1\r\n\r\n", "response not empty"),
        ("GET / HTTP/2.0\r\n\r\n", "response not empty"),
        ("GET / HTTP/1.1\r\n", "response not empty"),
    ];

    for (request, expected) in test_cases {
        let response = server
            .send_request(request)
            .expect("Failed to get response");
        assert!(!response.is_empty(), "Failed for {}: {}", request, expected);
    }
}
