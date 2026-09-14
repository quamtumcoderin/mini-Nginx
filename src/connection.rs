use crate::error::ServerError;
use crate::http::request::HttpRequest;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, warn};

pub struct Connection {
    stream: TcpStream
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub async fn process(&mut self) -> Result<(), ServerError> {
        let mut buffer = [0; 1024];

        let bytes_read = self.stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            return Ok(());
        }

        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

        if let Some(line) = request_str.lines().next() {
            match HttpRequest::parse_line(line) {
                Ok(req) => {
                    info!("request method={} path={} version={}", req.method, req.path, req.version);

                    // TODO: Delegar al Router (Próxima etapa).
                    let dummy_response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    self.stream.write_all(dummy_response.as_bytes()).await?;
                }
                Err(e) => {
                    warn!("Error parseando la petición: {}", e);
                    let bad_request = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    self.stream.write_all(bad_request.as_bytes()).await?;
                }
            }
        }

        Ok(())
    }
}