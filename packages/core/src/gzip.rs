use std::error::Error;
use std::io::Write;

use crate::filelike::FileLike;
use flate2::write::GzEncoder;
use flate2::Compression;

pub struct Gzip;

impl Gzip {
    pub fn compress(file: &FileLike) -> Result<Vec<u8>, Box<dyn Error>> {
        match file {
            FileLike::TextFile(file) => {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(file.as_bytes())?;
                let compressed_data = encoder.finish()?;
                Ok(compressed_data)
            }
            FileLike::ImageFile(image_file) => {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder
                    .write_all(&image_file)
                    .expect("Failed to gzip image data");
                let gzipped_data = encoder.finish()?;
                Ok(gzipped_data)
            }
        }
    }
}
