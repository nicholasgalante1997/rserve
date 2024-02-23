use std::io::{Error, Write};

use crate::logger::Logger;
use flate2::write::GzEncoder;
use flate2::Compression;

pub struct Gzip;

impl Gzip {
    pub fn compress(file_data: &str) -> Result<Vec<u8>, Error> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        if let Err(e) = encoder.write_all(file_data.as_bytes()) {
            Logger::error(&format!("{:#?}", e));
            return Err(e);
        };
        let compressed_data = encoder.finish()?;
        Ok(compressed_data)
    }
}
