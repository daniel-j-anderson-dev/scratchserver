mod http;

use http::{Request, Response};

use std::{
    io::{BufRead, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:8000")?;

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

        println!("raw: {:?}\n{:?}\n", raw_request, request);
    }

    return Ok(());
}

pub fn split_stream(
    possible_stream: Result<TcpStream, std::io::Error>,
) -> Option<(BufReader<TcpStream>, BufWriter<TcpStream>)> {
    return possible_stream.ok().and_then(|stream| {
        stream
            .try_clone()
            .ok()
            .map(|clone| (BufReader::new(clone), BufWriter::new(stream)))
    });
}
