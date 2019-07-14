use std::io;
use std::net::TcpListener;

#[macro_use]
extern crate log;

use rust_http_server;

fn main() -> Result<(), failure::Error> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Starging server...");
    let listener = TcpListener::bind("127.0.0.1:8888")?;
    info!("Server started!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match rust_http_server::handle_client(stream) {
                Err(e) => error!("Error handling client: {}", e),
                _ => (),
            },
            Err(e) => error!("Connection failed: {}", e),
        }
    }
    Ok(())
}
