use scratchserver::{http::{Request, Response}, split_stream};

use std::{
    io::BufRead,
    net::TcpListener,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::args().nth(1).unwrap_or("localhost:8000".to_string());
    let listener = TcpListener::bind(&ip)?;

    println!("\n------| Listening on http://localhost:8000 |------");

    for (mut reader, writer) in listener.incoming().filter_map(split_stream) {
        println!(
            "Connection established\nPeer Address: {:?}",
            reader.get_ref().peer_addr()
        );

        let mut raw_request = String::new();
        match reader.read_line(&mut raw_request) {
            Ok(_bytes_read) => (),
            Err(error) => {
                dbg!(raw_request);
                println!("Failed to read from client; {}", error);
                continue;
            }
        };

        let request = match raw_request.parse::<Request>() {
            Ok(request) => request,
            Err(error) => {
                println!("Failed to parse request\n{}\n", error);
                continue;
            }
        };

        println!("{:?}\nraw: {}", request, raw_request);
    }

    return Ok(());
}

