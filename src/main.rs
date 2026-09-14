mod config;
mod connection;
mod error;
mod http;
mod logging;
mod router;
mod server;
mod static_files;

use crate::server::Server;

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    logging::init();

    let server = Server::new("127.0.0.1", 8080);
    server.run().await?;

    Ok(())
}