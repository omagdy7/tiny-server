use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write};

pub fn encode_gzip_string(input: &str) -> std::io::Result<Vec<u8>> {
    // Create a buffer to hold the compressed data
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    // Write the input string into the encoder
    encoder.write_all(input.as_bytes())?;

    // Finish the encoding process and get the compressed data
    let compressed_data = encoder.finish()?;

    Ok(compressed_data)
}

pub fn save_bytes_to_file(bytes: &[u8], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(bytes)?;
    Ok(())
}

pub fn read_file_as_bytes(path: &str) -> io::Result<Vec<u8>> {
    // Open the file in read-only mode
    let mut file = File::open(path)?;

    // Create a buffer to hold the file contents
    let mut buffer = Vec::new();

    // Read the file contents into the buffer
    file.read_to_end(&mut buffer)?;

    // Return the buffer
    Ok(buffer)
}
