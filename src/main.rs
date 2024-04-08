use std::net::TcpListener;

use jiro::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    run(listener)?.await
}