use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1")?;

    for mut stream in listener.incoming().flatten() {
        let mut buffer = [0; 512];
        if let Ok(bytes) = stream.read(&mut buffer) {
            if let Ok(message) = bincode::deserialize::<String>(&buffer[0..bytes]) {
                println!("{message}");
                if let Ok(serialized) = bincode::serialize(&String::from("pong")) {
                    if let Err(e) = stream.write_all(&serialized) {
                        eprintln!("{e}");
                    }
                }
            }
        }
    }
    Ok(())
}
