pub enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "HTTP/1.1 200 OK",
            HttpStatus::NotFound => "HTTP/1.1 404 Not Found",
        }
    }
}
