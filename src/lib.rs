use std::{io::{BufReader, BufWriter}, net::TcpStream};

pub mod http;

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