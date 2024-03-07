use scratchserver::{handle_connection, split_stream};

use color_eyre::{owo_colors::OwoColorize, Report};
use std::{env, net::TcpListener};

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let ip = env::args().nth(1).unwrap_or("localhost:8000".to_string());
    let listener = TcpListener::bind(&ip)?;

    println!("\n------| Listening on http://localhost:8000 |------");

    for connection in listener.incoming().filter_map(split_stream) {
        if let Err(error) = handle_connection(connection) {
            println!("Error handling connection: {}", error.on_red());
        }
    }

    return Ok(());
}
