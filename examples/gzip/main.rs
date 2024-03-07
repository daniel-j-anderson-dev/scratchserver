use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    const FILE_DATA: &[u8] = include_bytes!("../../web/assets/ducks.gif");
    const OUTPUT_PATH: &str = "examples/gzip/ducks.gif.gz";

    // Create a Gzip encoder
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&FILE_DATA)?;
    let encoded_data = encoder.finish()?;

    // Write the compressed data to the output file
    let mut output = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(OUTPUT_PATH)?;

    output.write_all(&encoded_data)?;

    println!("Compression successful. Output file: {}", OUTPUT_PATH);

    Ok(())
}
