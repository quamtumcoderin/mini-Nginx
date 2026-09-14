use crate::error::ServerError;
use tokio::net::TcpListener;
use tracing::{error, info};

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub async fn run(&self) -> Result<(), ServerError> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;

        info!("Mini-Nginx listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((_stream, client_addr)) => {
                    info!("Accepted connection from {}", client_addr);
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}