pub mod my_http;
use crate::my_http::{Request, Response};

use color_eyre::{owo_colors::OwoColorize, Report};
use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

/// This assumes that the `web` directory lives in the root of the crate. and `cargo run` must be ran from the crate root
pub const SERVE_DIR: &'static str = "./web";

pub fn handle_connection(
    (reader, mut writer): (BufReader<TcpStream>, BufWriter<TcpStream>),
) -> Result<(), Report> {
    println!(
        "{}\n{}{:?}",
        "Connection established".green(),
        "Peer Address: ".green(),
        reader.get_ref().peer_addr().green()
    );

    let request = Request::try_from(reader)?;
    println!("{:#?}\n", request.bright_blue());

    let response = Response::try_from(request)?;
    if response.body_length() <= 1000 {
        println!("response:\n{}", response.on_blue());
    } else {
        println!("response:\n{}\n{}\n{}", response.start_line().on_blue(), response.headers().on_blue(), "body omitted for brevity".on_blue());
    }

    writer.write_all(response.to_string().as_bytes())?;

    return Ok(());
}

/// This function is meant to be used as the `predicate` for a [Iterator::filter_map] on [std::net::TcpListener::incoming]
/// # Example
/// ```rust
/// let listener = TcpListener::bind(ip)?;
/// for (reader, writer) in listener.incoming().filter_map(split_stream) {
///     // use the split stream
/// }
/// ```
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
