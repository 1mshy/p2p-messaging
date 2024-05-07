use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

use std::thread;
use tauri::command;

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Error reading from stream");
        if bytes_read == 0 { break; }
        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
        stream.write_all(&buffer[..bytes_read]).expect("Error writing to stream");
    }
}

async fn start_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn( || handle_client(stream));
            },
            Err(e) => { eprintln!("Error: {}", e); }
        }
    }
    Ok(())
}

async fn connect_to_peer(address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to peer at {}", address);

    let message = b"Hello from the other side!";
    stream.write_all(message)?;
    handle_client(stream).await;
    Ok(())
}

#[command]
pub async fn p2p_start() {
    let mode = "client";
    let address = "10.0.0.14:6000";

    match mode {
        "client" => {
            if let Err(e) = connect_to_peer(address).await {
                eprintln!("Failed to connect: {}", e);
            }
        },
        "server" => {
            if let Err(e) = connect_to_peer(address).await {
                eprintln!("Failed to connect: {}", e);
            }
        }
        _ => eprintln!("Invalid mode: {}", mode),
    }
}
