use std::{fs::File, io::Write};

use zip::{write::FileOptions, CompressionMethod};

use crate::models::errors::MatchMakerError;



pub fn save_to_zip(contents: String, file_name: &str) -> Result<(), MatchMakerError> {
    // Create or open the ZIP file
    let file = File::create(file_name).map_err(|e| MatchMakerError::IOError(e))?;
    let mut zip = zip::ZipWriter::new(file);

    // Specify the file name within the ZIP archive
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Start a new file inside the zip
    zip.start_file(file_name.replace(".zip", ".txt"), options)
        .map_err(|e| MatchMakerError::ZippingError(e.into()))?;

    // Write the game output to the file inside the zip
    zip.write_all(contents.as_bytes())
        .map_err(|e| MatchMakerError::IOError(e))?;

    // Finish writing the zip file
    zip.finish().map_err(|e| MatchMakerError::ZippingError(e.into()))?;
    Ok(())
}