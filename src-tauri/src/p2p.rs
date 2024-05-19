use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::sync::Arc;
use tauri::{command, Error, State};
use tokio::sync::Mutex;

pub(crate) struct AppState {
    pub(crate) connection: Arc<Mutex<Option<Arc<Mutex<TcpStream>>>>>,
}


async fn handle_client(stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = {
            let mut stream = stream.lock().await;
            stream.read(&mut buffer).expect("Error reading from stream")
        };
        if bytes_read == 0 { break; }
        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
        let _ = {
            let mut stream = stream.lock().await;
            stream.write_all(&buffer[..bytes_read]).expect("Error writing to stream")
        };
    }
}
async fn start_server(address: &str) -> io::Result<()> {
    let listener = std::net::TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let message = b"Hello this is the server how may I help you?";
                stream.write_all(message).expect("Failed to write to the connection");
                let stream = Arc::new(Mutex::new(stream));
                tokio::spawn(handle_client(stream));
            },
            Err(e) => { eprintln!("Error: {}", e); }
        }
    }
    Ok(())
}

async fn connect_to_peer_async(address: &str) -> io::Result<Arc<Mutex<TcpStream>>> {
    match TcpStream::connect(address) {
        Ok(stream) => {
            println!("Connected to peer at {}", address);
            let stream = Arc::new(Mutex::new(stream));
            tokio::spawn(handle_client(stream.clone()));
            Ok(stream)
        },
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", address, e);
            Err(e)
        }
    }
}
#[command]
pub async fn send_message(message: &str, state: State<'_, AppState>) -> Result<(), Error> {
    let conn = state.connection.lock().await;
    if let Some(stream) = &*conn {
        let mut stream = stream.lock().await;
        if let Err(e) = stream.write_all(message.as_bytes()) {
            eprintln!("Failed to send message: {}", e);
            return Err(Error::from(e));
        }
    } else {
        eprintln!("No connection available");
        return Err(Error::from(io::Error::new(io::ErrorKind::NotConnected, "No connection available")));
    }
    Ok(())
}

#[command]
pub async fn p2p_start() -> Result<(), Error> {
    let mode = "server";
    let address = "10.0.0.12:5555";
    println!("Connecting maybe");
    match mode {
        "client" => {
            if let Err(e) = connect_to_peer_async(address).await {
                eprintln!("Failed to connect: {}", e);
            }
        },
        "server" => {
            if let Err(e) = start_server(address).await {
                eprintln!("Failed to connect: {}", e);
            }
        }
        _ => eprintln!("Invalid mode: {}", mode),
    }
    Ok(())
}
