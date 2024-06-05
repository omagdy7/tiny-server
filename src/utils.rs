use std::fs::File;
use std::io::{self, Read, Write};

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
