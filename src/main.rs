use std::net::{TcpListener, TcpStream};
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let data = b"lol hello\n";
    stream.write_all(data).unwrap();
}

fn main() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}